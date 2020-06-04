//! Implementation of a blocking delay
//!
//! The MPU9250 driver needs a handle to a type
//! that can implement delay. This provides an
//! implementation of that based on the BSP's
//! SYSTICK.

use embedded_hal::blocking::delay;
use teensy4_bsp as bsp;

pub struct SystickDelay;

impl delay::DelayMs<u8> for SystickDelay {
    fn delay_ms(&mut self, ms: u8) {
        bsp::delay(ms as _);
    }
}
