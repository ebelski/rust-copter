//! An example of interfacing the MPU9250 on a Teensy 4.
//!
//! # Getting started
//!
//! - Attach the MPU's SCL pin to the Teensy 4's pin 16.
//! - Attach the MPU's SDA pin to the Teensy 4's pin 17.
//! - Ensure the MPU and the Teensy 4 are on the same ground
//! - Power the MPU from the Teensy 4's 3V3 output.

// We cannot use the Rust standard library,
// since the Rust standard library assumes an operating system.
// The Teensy 4 doesn't have an OS!
#![no_std]
// We're going to write our own main() function, rather than
// using the normal, 'special' main() function.
#![no_main]

// -------
// Modules
// -------
mod control;
mod delay;

// -------
// Imports
// -------
extern crate panic_halt;
use delay::SystickDelay;
use mpu9250::Mpu9250;
use teensy4_bsp as bsp; // Aliasing teensy4_bsp as bsp for convenience

/// System initialization
///
/// This is the fist thing that runs. By this point, the processor
/// is ready to go. Most importantly, we can use floating-point
/// operations by the time main() is called.
#[bsp::rt::entry]
fn main() -> ! {
    // Initializes system peripherals, and exposes them as a Peripherals
    // object. This can only be called once!
    let mut peripherals = bsp::Peripherals::take().unwrap();

    // We'll set up the logging system, since it will be nice
    // to print things out. See the usb demo in the teensy4-rs
    // repo for the five different log levels we can use:
    // https://github.com/mciantyre/teensy4-rs/blob/master/teensy4-examples/src/usb.rs
    peripherals.log.init(bsp::log::Config {
        // Only keep log messages from this executable.
        // The HAL and BSP might print debug log messages,
        // but we're not interested in those here.
        filters: &[
            ("teensy4_rs_mpu9250", None),
            ("teensy4_rs_mpu9250::control", None),
        ],
        ..Default::default()
    });

    // Run the main clock at 600MHz
    peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ, // <-- 600MHz constant
        &mut peripherals.ccm.handle, // Handle to the clock control module (CCM)
        &mut peripherals.dcdc,       // Handle to the power control module (DCDC)
    );

    // ---------
    // I2C setup
    // ---------
    let (_, _, i2c3_builder, _) = peripherals.i2c.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::i2c::ClockSelect::OSC, // Use the 24MHz oscillator as the peripheral clock source
        bsp::hal::ccm::i2c::PrescalarSelect::DIVIDE_1, // Divide that 24MHz clock by 1
    );

    // Instantiate an I2C peripheral on pins 16 and 17.
    // The alt1() means that we're configuring the pin to be
    // used specifically for I2C functions.
    let i2c3 = i2c3_builder.build(peripherals.pins.p16.alt1(), peripherals.pins.p17.alt1());

    // ---------
    // MPU setup
    // ---------

    // Create an MPU object that will use the SYSTICK as its
    // delay implementation. It will interface an MPU9250 over
    // I2C.
    let mut systick_delay = SystickDelay;
    let mut mpu = match Mpu9250::marg_default(i2c3, &mut systick_delay) {
        // Damn, something went wrong when connecting to the MPU!
        Err(err) => {
            log::error!("Unable to create MPU9250: {:?}", err);
            loop {
                // This is it, we stop the example here.
                core::sync::atomic::spin_loop_hint()
            }
        }
        // Connected OK to the MPU!
        Ok(mpu) => mpu,
    };

    // A brief delay, before we start logging things.
    // Gives you a chance to open up your terminal...
    bsp::delay(7_000); // 7 seconds

    // Sanity check: log WHO_AM_I. Should see 0x71.
    let who_am_i = mpu.who_am_i().unwrap();
    log::info!("WHO_AM_I = 0x{:X}", who_am_i);

    log::trace!("Starting poll and control loop...");
    bsp::delay(1_000);
    loop {
        bsp::delay(control::SAMPLING_DELAY_MILLISECONDS);
        // Get the reading from the MPU
        let marg: mpu9250::MargMeasurements<control::Triplet<f32>> = match mpu.all() {
            // Ahh, something bad happened! Let's try again after waiting no less than
            // 1 second...
            Err(err) => {
                log::warn!("Error when querying for MPU 9DOF reading: {:?}", err);
                bsp::delay(1_000.max(control::SAMPLING_DELAY_MILLISECONDS));
                continue;
            }
            // Got eeeeemmmmmmm
            Ok(marg) => marg,
        };
        // Convert their readings into our readings
        let reading = control::Readings {
            acc: marg.accel,
            gyro: marg.gyro,
            mag: marg.mag,
            temp: marg.temp,
        };
        // Do things with those readings.
        control::on_reading(&reading);
    }
}
