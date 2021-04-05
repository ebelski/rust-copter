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
// This seems to be a bad lint. The patter is valid
// for a non_exhaustive struct.
#![allow(clippy::field_reassign_with_default)]

// -------
// Imports
// -------
extern crate panic_halt;
use bsp::hal::i2c::ClockSpeed;
use motion_sensor::*;
use teensy4_bsp as bsp; // Aliasing teensy4_bsp as bsp for convenience

const I2C_CLOCK_SPEED: ClockSpeed = ClockSpeed::KHz400;

/// System initialization
///
/// This is the fist thing that runs. By this point, the processor
/// is ready to go. Most importantly, we can use floating-point
/// operations by the time main() is called.
#[cortex_m_rt::entry]
fn main() -> ! {
    // Initializes system peripherals, and exposes them as a Peripherals
    // object. This can only be called once!
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(core_peripherals.SYST);
    let pins = bsp::t40::into_pins(peripherals.iomuxc);

    // We'll set up the logging system, since it will be nice
    // to print things out. See the usb demo in the teensy4-rs
    // repo for the five different log levels we can use:
    // https://github.com/mciantyre/teensy4-rs/blob/master/teensy4-examples/src/usb.rs
    bsp::usb::init(
        &systick,
        bsp::usb::LoggingConfig {
            ..Default::default()
        },
    )
    .unwrap();

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
        bsp::hal::ccm::i2c::PrescalarSelect::DIVIDE_3, // Divide that 24MHz clock by 3
    );

    // Instantiate an I2C peripheral on pins 16 and 17.
    // The alt1() means that we're configuring the pin to be
    // used specifically for I2C functions.
    systick.delay(5000);
    let mut i2c3 = i2c3_builder.build(pins.p16, pins.p17);
    // Set the I2C clock speed. If this returns an error, log it and stop.
    match i2c3.set_clock_speed(I2C_CLOCK_SPEED) {
        Ok(_) => (),
        Err(err) => {
            log::error!(
                "Unable to set I2C clock speed to {:?}: {:?}",
                I2C_CLOCK_SPEED,
                err
            );
            loop {
                // Nothing more to do
                core::hint::spin_loop();
            }
        }
    }

    // ---------
    // MPU setup
    // ---------

    // Create an MPU object that will use the SYSTICK as its
    // delay implementation. It will interface an MPU9250 over
    // I2C.
    log::info!("Creating MPU9250...");
    systick.delay(5000);

    let mut config = invensense_mpu::Config::default();
    config.accel_scale = invensense_mpu::regs::ACCEL_FS_SEL::G8;
    config.mag_control = invensense_mpu::regs::CNTL1 {
        mode: invensense_mpu::regs::CNTL1_MODE::CONTINUOUS_2,
        ..Default::default()
    };
    let mut sensor = match invensense_mpu::i2c::new(i2c3, &mut systick, &config) {
        // Damn, something went wrong when connecting to the MPU!
        Err(err) => {
            log::error!("Unable to create MPU9250: {:?}", err);
            loop {
                // This is it, we stop the example here.
                core::hint::spin_loop()
            }
        }
        // Connected OK to the MPU!
        Ok(mpu) => mpu,
    };

    // A brief delay, before we start logging things.
    // Gives you a chance to open up your terminal...
    systick.delay(7_000); // 7 seconds

    // Sanity check: log WHO_AM_I. Should see 0x71.
    let who_am_i = sensor.mpu9250_who_am_i().unwrap();
    log::info!("WHO_AM_I = 0x{:X}", who_am_i);

    log::trace!("Starting poll and control loop...");
    systick.delay(1_000);
    loop {
        log::info!("ACC {:?}", sensor.accelerometer().unwrap());
        log::info!("GYRO {:?}", sensor.gyroscope().unwrap());
        log::info!("MAG {:?}", sensor.magnetometer().unwrap());

        systick.delay(250);
    }
}
