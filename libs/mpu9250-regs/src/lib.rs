//! MPU9250 Registers
//!
//! The crate exposes register addresses for addressing the MPU9250 acceleromter
//! and gyroscope, as well as the AK8963 magnetometer. See the documentation in
//! each module for more information.

#![no_std]

pub mod ak8963;
pub mod mpu9250;
