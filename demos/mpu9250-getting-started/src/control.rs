//! Some kind of control loop
//!
//! The control loop defines the polling interval and does processing
//! on the MPU's 9DOF measurements. Provide an implementation of the
//! `on_reading` function to work with MPU readings.

use super::ClockSpeed;
use core::time::Duration; // I2C clock selection

/// Convenience for accessing data on x, y, and z axes.
/// The `T` is a generic parameter. In practice, we'll
/// work with 32-bit floats, `f32`.
#[derive(Default, Debug, Clone, Copy)]
pub struct Triplet<T> {
    x: T,
    y: T,
    z: T,
}

/// Measurements from the MPU9250
#[derive(Default, Debug, Clone, Copy)]
pub struct Readings {
    /// Accelerometer readings in x, y, and z
    pub acc: Triplet<f32>,
    /// Gyroscope readings in x, y, and z
    pub gyro: Triplet<f32>,
    /// Magnetometer readings in x, y, and z
    pub mag: Triplet<f32>,
    /// Temperature reading
    pub temp: f32,
}

/// Select your I2C clock speed. Options are
///
/// - `KHz100` => 100 KHz clock
/// - `KHz400` => 400 KHz clock
/// - `MHz1` => 1 MHz clock (unsupported on the MPU)
///
/// The faster the clock, the faster we can query on the I2C bus
pub const I2C_CLOCK_SPEED: ClockSpeed = ClockSpeed::KHz400;

/// How long we should wait in between polling. This
/// defines how frequently the `on_reading` function
/// is called. Feel free to change me to something larger
/// or smaller! It should be greater than 0.
pub const SAMPLING_DELAY_MILLISECONDS: u32 = 10;

/// Called every SAMPLING_DELAY_MILLISECONDS in the sampling loop.
/// The provided `readings` is the latest reading from the MPU.
/// The `reading_duration` describes how long it took to acquire
/// all readings via the I2C call. If the timer expired, the input is
/// `None`.
pub fn on_reading(readings: &Readings, reading_duration: Option<Duration>) {
    // Just printing the most-recent reading for now.
    // TODO make me do something cool!
    log::info!(
        "Acc: x = {:.3}, y = {:.3}, z = {:.3}", // 3 decimal points
        readings.acc.x,
        readings.acc.y,
        readings.acc.z,
    );

    if let Some(duration) = reading_duration {
        log::trace!("Took {:?} to query for all readings", duration);
    }
}

// -------
// Details
// -------

// Bridges the MPU9250 library's interface so that
// we can read directly into our Triplet types.
impl From<[f32; 3]> for Triplet<f32> {
    fn from([x, y, z]: [f32; 3]) -> Triplet<f32> {
        Triplet { x, y, z }
    }
}
