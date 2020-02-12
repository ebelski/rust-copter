//! PWM control example
//!
//! The example lets us change PWM outputs from user input. There are four
//! PWM outputs, identified by the letters A through D:
//!
//! | PWM Output | Teensy 4 Pin | PWM instance |
//! | ---------- | ------------ | ------------ |
//! |     A      |      6       |  `PWM2_2_A`  |
//! |     B      |      7       |  `PWM1_3_B`  |
//! |     C      |      8       |  `PWM1_3_A`  |
//! |     D      |      9       |  `PWM2_2_B`  |
//!
//! To set the duty cycle of a PWM output, use the addressing schema
//!
//! ```text
//! O.ppp\r
//! ````
//!
//! where
//!
//! - `O` is one of the four PWM output letters
//! - `ppp` is a percentage from 0 to 100. The software will require inputs within this range.
//! - `\r` is a carriage return character
//!
//! Example: to set the duty cycle of PWM output `C` to 37%, type `C.37`, then press `ENTER` on your
//! keyboard.
//!
//! If you start to enter an invalid number, just press ENTER to submit it, and let the parser fail.
//!
//! To read back the duty cycles for all PWM outputs, send 'r' (lower case 'R').
//! 
//! Press the SPACE bar to reset all PWM outputs to 0% duty cycle. Use this in case of emergency...

#![no_std]
#![no_main]

mod parser;

extern crate panic_halt;

use bsp::rt::entry;
use core::time::Duration;
use embedded_hal::{digital::v2::ToggleableOutputPin, timer::CountDown, PwmPin};
use parser::{Command, Output, Parser};
use teensy4_bsp as bsp;

/// Change me to modify the PWM switching frequency.
const SWITCHING_FREQUENCY_HZ: u64 = 1_000;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut led = peripherals.led;

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
    let (mut output_a, mut output_d) = pwm2
        .outputs(
            peripherals.pins.p6.alt2(),
            peripherals.pins.p9.alt2(),
            bsp::hal::pwm::Timing {
                clock_select: bsp::hal::ccm::pwm::ClockSelect::IPG(ipg_hz),
                prescalar: bsp::hal::ccm::pwm::Prescalar::PRSC_5,
                switching_period,
            },
        )
        .unwrap()
        .split();

    // Configure pins 7 and 8 as PWM outputs
    let (mut output_c, mut output_b) = pwm1
        .outputs(
            peripherals.pins.p8.alt6(),
            peripherals.pins.p7.alt6(),
            bsp::hal::pwm::Timing {
                clock_select: bsp::hal::ccm::pwm::ClockSelect::IPG(ipg_hz),
                prescalar: bsp::hal::ccm::pwm::Prescalar::PRSC_5,
                switching_period,
            },
        )
        .unwrap()
        .split();

    output_a.enable();
    output_b.enable();
    output_c.enable();
    output_d.enable();

    // Set up the USB stack, and use the USB reader for parsing commands
    let usb_reader = peripherals.usb.init(Default::default());
    let mut parser = Parser::new(usb_reader);

    let blink_period = pwm_to_blink_period(&[&output_a, &output_b, &output_c, &output_d]);
    led_timer.start(blink_period);

    loop {
        if let Ok(()) = led_timer.wait() {
            led.toggle().unwrap();
        }

        match parser.parse() {
            // Parser has not found any command; it needs more inputs
            Ok(None) => bsp::delay(10),
            // User wants to reset all duty cycles
            Ok(Some(Command::ResetDuty)) => {
                output_a.set_duty(0);
                output_b.set_duty(0);
                output_c.set_duty(0);
                output_d.set_duty(0);
                log::info!("Reset all duty cycles");
                let blink_period =
                    pwm_to_blink_period(&[&output_a, &output_b, &output_c, &output_d]);
                led_timer.start(blink_period);
            }
            // User wants to read all the duty cycles
            Ok(Some(Command::ReadDuty)) => {
                log::info!("'A' = {}", duty_to_percent(output_a.get_duty()));
                log::info!("'B' = {}", duty_to_percent(output_b.get_duty()));
                log::info!("'C' = {}", duty_to_percent(output_c.get_duty()));
                log::info!("'D' = {}", duty_to_percent(output_d.get_duty()));
            }
            // User has set a duty cycle for an output PWM
            Ok(Some(Command::SetDuty { output, percent })) => {
                let pwm: &mut dyn PwmPin<Duty = u16> = match output {
                    Output::A => &mut output_a,
                    Output::B => &mut output_b,
                    Output::C => &mut output_c,
                    Output::D => &mut output_d,
                };

                log::info!("SETTING '{:?}' = {}% duty cycle", output, percent);
                let duty = percent_to_duty(percent);
                pwm.set_duty(duty);

                let blink_period =
                    pwm_to_blink_period(&[&output_a, &output_b, &output_c, &output_d]);
                led_timer.start(blink_period);
            }
            // Parser detected an error
            Err(err) => {
                log::warn!("{:?}", err);
            }
        };
    }
}

/// Converts a percentage to a 16-bit duty cycle
fn percent_to_duty(pct: f64) -> u16 {
    ((u16::max_value() as f64) * (pct / 100.0f64)) as u16
}

/// Converts a 16-bit duty cycle to a percentage
fn duty_to_percent(duty: u16) -> f64 {
    ((duty as f64) * 100.0f64) / (u16::max_value() as f64)
}

/// Compute the rate at which we should blink the LED based on the
/// PWM duty cycles. Defines a duration based on the highest PWM
/// duty cycle.
fn pwm_to_blink_period(pwms: &[&dyn PwmPin<Duty = u16>]) -> Duration {
    const SLOWEST_BLINK_NS: i64 = 2_000_000_000;
    const FASTEST_BLINK_NS: i64 = SLOWEST_BLINK_NS / 100;
    if let Some(duty) = pwms.iter().map(|pwm| pwm.get_duty()).max() {
        let ns = (duty as i64) * (FASTEST_BLINK_NS - SLOWEST_BLINK_NS) / 0xFFFF + SLOWEST_BLINK_NS;
        Duration::from_nanos(ns as u64)
    } else {
        Duration::from_nanos(SLOWEST_BLINK_NS as u64)
    }
}
