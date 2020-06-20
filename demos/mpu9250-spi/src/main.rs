//! Pinout:
//!
//! - Teensy 4 Pin 13 (SCK) to MPU's SCL (Note that we lose the LED here)
//! - Teensy 4 Pin 11 (MOSI) to MPU's SDA/SDI
//! - Teensy 4 Pin 12 (MISO) to MPU's AD0/SDO
//! - Teensy 4 Pin 10 (PSC0) to MPU's NCS

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt::entry;
use motion_sensor::*;
use teensy4_bsp as bsp;

const SPI_BAUD_RATE_HZ: u32 = 1_000_000;

pub struct SystickDelay;

impl embedded_hal::blocking::delay::DelayMs<u8> for SystickDelay {
    fn delay_ms(&mut self, ms: u8) {
        bsp::delay(ms as _);
    }
}

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    peripherals.usb.init(Default::default());

    peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    bsp::delay(5000);
    log::info!("Initializing SPI4 clocks...");

    let (_, _, _, spi4_builder) = peripherals.spi.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::spi::ClockSelect::Pll2,
        bsp::hal::ccm::spi::PrescalarSelect::LPSPI_PODF_5,
    );

    log::info!("Constructing SPI4 peripheral...");
    let mut spi4 = spi4_builder.build(
        peripherals.pins.p11.alt3(),
        peripherals.pins.p12.alt3(),
        peripherals.pins.p13.alt3(),
    );

    match spi4.set_clock_speed(bsp::hal::spi::ClockSpeed(SPI_BAUD_RATE_HZ)) {
        Ok(()) => {
            log::info!("Set clock speed to {}Hz", SPI_BAUD_RATE_HZ);
        }
        Err(err) => {
            log::error!(
                "Unable to set clock speed to {}Hz: {:?}",
                SPI_BAUD_RATE_HZ,
                err
            );
            loop {
                core::sync::atomic::spin_loop_hint()
            }
        }
    };

    spi4.enable_chip_select_0(peripherals.pins.p10.alt3());
    log::info!("Waiting a few seconds before querying MPU9250...");
    bsp::delay(4000);

    let mut sensor = match invensense_mpu::spi::SPI::new(spi4, &mut SystickDelay) {
        Ok(sensor) => sensor,
        Err(err) => {
            log::error!("Error when constructing MP9250: {:?}", err);
            loop {
                core::sync::atomic::spin_loop_hint();
            }
        }
    };

    log::info!(
        "MPU9250 WHO_AM_I = {:#X}",
        sensor.mpu9250_who_am_i().unwrap()
    );
    log::info!("AK8963 WHO_AM_I = {:#X}", sensor.ak8963_who_am_i().unwrap());
    loop {
        core::sync::atomic::spin_loop_hint();
        log::info!("ACC {:?}", sensor.accelerometer().unwrap());
        log::info!("GYRO {:?}", sensor.gyroscope().unwrap());
        log::info!("MAG {:?}", sensor.magnetometer().unwrap());

        bsp::delay(250);
    }
}
