//! Some kind of control loop
//!
//! The control loop defines the polling interval and does processing
//! on the MPU's 9DOF measurements. Provide an implementation of the
//! `on_reading` function to work with MPU readings.

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

/// How long we should wait in between polling. This
/// defines how frequently the `on_reading` function
/// is called. Feel free to change me to something larger
/// or smaller! It should be greater than 0.
pub const SAMPLING_DELAY_MILLISECONDS: u32 = 250;

/// Called every SAMPLING_DELAY_MILLISECONDS in the sampling loop.
/// The provided `readings` is the latest reading from the MPU.
pub fn on_reading(readings: &Readings) {
    // Just printing the most-recent reading for now.
    // TODO make me do something cool!
    log::info!(
        "Acc: x = {:.3}, y = {:.3}, z = {:.3}", // 2 decimal points
        readings.acc.x,
        readings.acc.y,
        readings.acc.z
    );
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
