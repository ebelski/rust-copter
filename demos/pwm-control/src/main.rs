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
//! - `ppp` is a percentage from 0 to 100. The software will clamp the percentage to this range.
//! - `\r` is a carriage return character
//!
//! Example: to set the duty cycle of PWM output `C` to 37%, type `C.37`, then press `ENTER` on your
//! keyboard.
//!
//! If you start to enter an invalid number, just press ENTER to submit it, and let the parser fail.
//!
//! To read back the duty cycles for all PWM outputs, send 'r'.

#![no_std]
#![no_main]

mod parser;

extern crate panic_halt;

use bsp::rt::entry;
use embedded_hal::PwmPin;
use parser::{Command, Output, Parser};
use teensy4_bsp as bsp;

/// Change me to modify the PWM switching frequency.
const SWITCHING_FREQUENCY_HZ: u64 = 1_000;

/// Converts a percentage to a 16-bit duty cycle
fn percent_to_duty(pct: f64) -> u16 {
    ((u16::max_value() as f64) * (pct / 100.0f64)) as u16
}

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();

    // Initialize the ARM and IPG clocks. The PWM module runs on the IPG clock.
    let (_, ipg_hz) = peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    // Enable clocks to the PWM2 module
    let mut pwm2 = peripherals.pwm2.clock(&mut peripherals.ccm.handle);

    // Compute the switching period from the user-configurable SWITCHING_FREQUENCY_HZ.
    let switching_period =
        core::time::Duration::from_nanos(1_000_000_000u64 / SWITCHING_FREQUENCY_HZ);

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

    output_a.enable();
    output_d.enable();

    // Set up the USB stack, and use the USB reader for parsing commands
    let usb_reader = peripherals.usb.init(Default::default());
    let mut parser = Parser::new(usb_reader);

    loop {
        match parser.parse() {
            Ok(None) => {
                // No command available
                bsp::delay(10);
            }
            Ok(Some(Command::ReadDuty)) => {
                log::warn!("NOT IMPLEMENTED");
            }
            Ok(Some(Command::SetDuty { output, percent })) => {
                let pwm: &mut dyn PwmPin<Duty = u16> = match output {
                    Output::A => &mut output_a,
                    Output::D => &mut output_d,
                    _ => {
                        log::warn!("OUTPUT B AND C NOT IMPLEMENTED");
                        continue;
                    }
                };

                log::info!("SETTING '{:?}' = {}% duty cycle", output, percent);
                let duty = percent_to_duty(percent);
                pwm.set_duty(duty);
            }
            Err(err) => {
                log::warn!("{:?}", err);
            }
        };
    }
}
