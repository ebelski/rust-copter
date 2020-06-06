//! Interfaces and common types for electronic speed controls (ESC)

#![no_std]

/// Quadcoptor motor identifier
///
/// The mapping of a physical motor to an identifier is defined by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuadMotor {
    /// Motor 'A'
    A,
    /// Motor 'B'
    B,
    /// Motor 'C'
    C,
    /// Motor 'D'
    D,
}

/// An electronic speed control
pub trait ESC {
    /// Identifiers for motors
    type Motor: Copy;

    /// Returns the current throttle value for `motor`
    ///
    /// - `0.0` is no throttle
    /// - `1.0` is 100% throttle
    ///
    /// Implementations will never return anything out of that range.
    fn throttle(&self, motor: Self::Motor) -> f32;

    /// Set the throttle percentage for a motor
    ///
    /// - `0.0` is no throttle
    /// - `1.0` is 100% throttle
    ///
    /// Implementations will clamp `percent` values between `0.0` and `1.0`.
    fn set_throttle(&mut self, motor: Self::Motor, percent: f32);

    /// Set the throttle for each motor
    ///
    /// The default implementation simply calls `throttle()` for each `(motor, percent)` pair in
    /// the collection. But, implementations may opt to set the PWM duty cycle for each output
    /// in a single transaction.
    fn set_throttle_group(&mut self, percents: &[(Self::Motor, f32)]) {
        percents
            .iter()
            .for_each(|(motor, percent)| self.set_throttle(*motor, *percent))
    }
}
