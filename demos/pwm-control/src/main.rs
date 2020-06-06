//! PWM control example
//!
//! The example lets us change PWM outputs from user input. There are four
//! PWM outputs, identified by the letters A through D:
//!
//! | PWM Output | Teensy 4 Pin | PWM instance |
//! | ---------- | ------------ | ------------ |
//! |     C      |      6       |  `PWM2_2_A`  |
//! |     B      |      7       |  `PWM1_3_B`  |
//! |     A      |      8       |  `PWM1_3_A`  |
//! |     D      |      9       |  `PWM2_2_B`  |
//!
//! To set the throttle commanded by a PWM output, use the addressing schema
//!
//! ```text
//! O.ppp\r
//! ````
//!
//! where
//!
//! - `O` is one of the four output letters
//! - `ppp` is a throttle percentage from 0 to 100. The software will require inputs within this range.
//! - `\r` is a carriage return character
//!
//! Example: to set the throttle for output `C` to 37%, type `C.37`, then press `ENTER` on your
//! keyboard.
//!
//! If you start to enter an invalid number, just press ENTER to submit it, and let the parser fail.
//!
//! To read back the throttle for all outputs, send 'r' (lower case 'R').
//!
//! Press the SPACE bar to reset all throttles to 0%. Use this in case of emergency...
//!
//! The demo implements a basic electronic speed control (ESC) PWM driver. The PWM duty cycle varies between
//! 1000us for 0% throttle to 2000us for 100% throttle speed.
//!
//! ```text
//!      1000us              2000us      1000us          1000us          1000us      
//!     +-------+       +--------------++-------+       +-------+       +-------+    
//!     |       |       |              ||       |       |       |       |       |    
//!     |       |       |              ||       |       |       |       |       |    
//!     |       |       |              ||       |       |       |       |       |    
//! ----+       +-------+              ++       +-------+       +-------+       +---...
//! ```
//!
//! _(Above) a quick pulse to 100% throttle in between commands to hold 0% throttle_
//!
//! Given that we need to hold a 2000us pulse to represent 100% throttle, the fastest switching frequency
//! is 1 / 2000us == 500Hz. At 500Hz switching frequency, 100% throttle maps to 100% duty cycle. Therefore,
//! 50% duty cycle represents 0% throttle. We're assuming that the ESC
//! can perfectly detect a 2000us pulse followed immediately by another 2000us pulse, where there may be
//! no delay between the two pulses.

#![no_std]
#![no_main]

mod parser;

extern crate panic_halt;

use bsp::rt::entry;
use core::time::Duration;
use embedded_hal::{
    digital::v2::{OutputPin, ToggleableOutputPin},
    timer::CountDown,
};
use parser::{Command, Parser};
use teensy4_bsp as bsp;

use esc::{QuadMotor, ESC};
use esc_imxrt1062::Slow as SlowESC;

const SWITCHING_FREQUENCY_HZ: u64 = 500;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut led = bsp::configure_led(&mut peripherals.gpr, peripherals.pins.p13);

    // Initialize the ARM and IPG clocks. The PWM module runs on the IPG clock.
    let (_, ipg_hz) = peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    // Use one of the PIT timers to blink the LED at 1Hz
    let pit_cfg = peripherals.ccm.perclk.configure(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_3,
        bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );
    let (mut led_timer, _, _, _) = peripherals.pit.clock(pit_cfg);

    // Enable clocks to the PWM modules
    let mut pwm1 = peripherals.pwm1.clock(&mut peripherals.ccm.handle);
    let mut pwm2 = peripherals.pwm2.clock(&mut peripherals.ccm.handle);

    // Compute the switching period from the user-configurable SWITCHING_FREQUENCY_HZ.
    let switching_period = Duration::from_nanos(1_000_000_000u64 / SWITCHING_FREQUENCY_HZ);

    // Configure pins 6 and 9 as PWM outputs
    let sm2 = pwm2
        .sm2
        .outputs(
            &mut pwm2.handle,
            peripherals.pins.p6.alt2(),
            peripherals.pins.p9.alt2(),
            bsp::hal::pwm::Timing {
                clock_select: bsp::hal::ccm::pwm::ClockSelect::IPG(ipg_hz),
                prescalar: bsp::hal::ccm::pwm::Prescalar::PRSC_5,
                switching_period,
            },
        )
        .unwrap();

    // Configure pins 7 and 8 as PWM outputs
    let sm3 = pwm1
        .sm3
        .outputs(
            &mut pwm1.handle,
            peripherals.pins.p8.alt6(),
            peripherals.pins.p7.alt6(),
            bsp::hal::pwm::Timing {
                clock_select: bsp::hal::ccm::pwm::ClockSelect::IPG(ipg_hz),
                prescalar: bsp::hal::ccm::pwm::Prescalar::PRSC_5,
                switching_period,
            },
        )
        .unwrap();

    let mut esc = SlowESC::new(pwm1.handle, sm3, pwm2.handle, sm2);

    // Set up the USB stack, and use the USB reader for parsing commands
    let usb_reader = peripherals.usb.init(Default::default());
    let mut parser = Parser::new(usb_reader);

    let blink_period = pwm_to_blink_period(&esc);
    led_timer.start(blink_period);

    loop {
        if let Ok(()) = led_timer.wait() {
            led.toggle().unwrap();
        }

        match parser.parse() {
            // Parser has not found any command; it needs more inputs
            Ok(None) => bsp::delay(10),
            // User wants to reset all duty cycles
            Ok(Some(Command::ResetThrottle)) => {
                esc.set_throttle_group(&[
                    (QuadMotor::A, 0.0),
                    (QuadMotor::B, 0.0),
                    (QuadMotor::C, 0.0),
                    (QuadMotor::D, 0.0),
                ]);
                log::info!("Reset all outputs to 0% throttle");
                let blink_period = pwm_to_blink_period(&esc);
                led_timer.start(blink_period);
            }
            // User wants to read all the throttle settings
            Ok(Some(Command::ReadThrottle)) => {
                log::info!("A = {}", esc.throttle(QuadMotor::A) * 100.0);
                log::info!("B = {}", esc.throttle(QuadMotor::B) * 100.0);
                log::info!("C = {}", esc.throttle(QuadMotor::C) * 100.0);
                log::info!("D = {}", esc.throttle(QuadMotor::D) * 100.0);
            }
            // User has set a throttle for an output
            Ok(Some(Command::SetThrottle { output, percent })) => {
                log::info!("SETTING '{:?}' = {}% throttle", output, percent);
                esc.set_throttle(output, percent / 100.0);

                let blink_period = pwm_to_blink_period(&esc);
                led_timer.start(blink_period);
            }
            Ok(Some(Command::KillSwitch)) => {
                esc.kill();

                log::warn!("------------------------------------");
                log::warn!("USER PRESSED THE KILL SWITCH");
                log::warn!("I've stopped all PWM outputs,");
                log::warn!("and I've stopped accepting commands.");
                log::warn!("Reset your system to start over.");
                log::warn!("------------------------------------");

                led.set_high().unwrap();
                loop {
                    bsp::delay(1_000);
                    cortex_m::asm::wfe();
                }
            }
            // Parser detected an error
            Err(err) => {
                log::warn!("{:?}", err);
            }
        };
    }
}

/// The minimum duty cycle for the ESC PWM protocol is 50% duty cycle.
/// Since the underlying PWM duty cycle spans all `u16` values, the minimum
/// duty cycle is half of that.
const MINIMUM_DUTY_CYCLE: u16 = u16::max_value() >> 1;

/// Converts a percentage to a 16-bit duty cycle that implements the ESC PWM protocol
fn percent_to_duty(pct: f64) -> u16 {
    ((MINIMUM_DUTY_CYCLE as f64) * (pct / 100.0f64)) as u16 + MINIMUM_DUTY_CYCLE
}

/// Compute the rate at which we should blink the LED based on the
/// PWM duty cycles. Defines a duration based on the highest PWM
/// duty cycle.
fn pwm_to_blink_period(esc: &dyn ESC<Motor = QuadMotor>) -> Duration {
    const SLOWEST_BLINK_NS: i64 = 2_000_000_000;
    const FASTEST_BLINK_NS: i64 = SLOWEST_BLINK_NS / 100;
    const MOTORS: [QuadMotor; 4] = [QuadMotor::A, QuadMotor::B, QuadMotor::C, QuadMotor::D];
    if let Some(duty) = MOTORS
        .iter()
        .map(|motor| percent_to_duty(100f64 * esc.throttle(*motor) as f64))
        .max()
    {
        let ns = (duty as i64) * (FASTEST_BLINK_NS - SLOWEST_BLINK_NS) / 0xFFFF + SLOWEST_BLINK_NS;
        Duration::from_nanos(ns as u64)
    } else {
        Duration::from_nanos(SLOWEST_BLINK_NS as u64)
    }
}
