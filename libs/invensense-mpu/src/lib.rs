//! Implementation of the `motion-sensor` interfaces for an Invensense MPU9250

#![no_std]

pub mod i2c;
pub mod spi;

/// Re-export the registers under a different name
mod regs {
    pub use mpu9250_regs::{
        ak8963::flags::*, ak8963::Regs as AK8963, ak8963::I2C_ADDRESS as AK8963_I2C_ADDRESS,
        mpu9250::flags::*, mpu9250::Regs as MPU9250, mpu9250::I2C_ADDRESS as MPU9250_I2C_ADDRESS,
    };
}

use core::fmt::Debug;

/// Device errors that could occur during initialization or communication
#[derive(Debug)]
pub enum Error<P> {
    LostArbitration,
    Nack,
    Timeout {
        attempts: u16,
        register: regs::AK8963,
        value: Option<u8>,
    },
    Peripheral(P),
    WhoAmI {
        expected: &'static [u8],
        actual: u8,
    },
}

impl<P> From<P> for Error<P> {
    fn from(err: P) -> Self {
        Error::Peripheral(err)
    }
}

/// Holds controller-side state of the MPU9250
pub struct Handle {}

/// An Invensense MPU
///
/// The `MPU` struct can be used to query readings from a physical
/// MPU.
pub struct MPU<T> {
    transport: T,
    handle: Handle,
}

impl<T> MPU<T>
where
    T: Transport,
{
    fn new(transport: T) -> Self {
        MPU {
            transport,
            handle: Handle {},
        }
    }
}

impl<T> MPU<T>
where
    T: Transport,
    T::Error: Debug,
{
    /// Query the MPU9250's `WHO_AM_I` register
    pub fn mpu9250_who_am_i(&mut self) -> Result<u8, Error<T::Error>> {
        self.transport.mpu9250_read(regs::MPU9250::WHO_AM_I)
    }

    /// Query the AK8963's `WHO_AM_I` register
    pub fn ak8963_who_am_i(&mut self) -> Result<u8, Error<T::Error>> {
        self.transport.ak8963_read(regs::AK8963::WIA)
    }
}

/// `Transport` lets us generalize device configuration across both
/// SPI and I2C interfaces
///
/// It should *not* be used for higher-speed data queries; those should be
/// implemented manually, using the best approach available for your peripheral.
pub trait Transport: private::Sealed {
    type Error;

    /// Read a register from the MPU9250
    fn mpu9250_read(&mut self, register: regs::MPU9250) -> Result<u8, Error<Self::Error>>;
    /// Write a value to an MPU9250 register
    fn mpu9250_write<B: Copy + Into<u8>>(
        &mut self,
        register: regs::MPU9250,
        value: B,
    ) -> Result<(), Error<Self::Error>>;
    /// Read an AK8963 register
    fn ak8963_read(&mut self, register: regs::AK8963) -> Result<u8, Error<Self::Error>>;
    /// Write an AK8963 register
    fn ak8963_write<B: Copy + Into<u8>>(
        &mut self,
        register: regs::AK8963,
        value: B,
    ) -> Result<(), Error<Self::Error>>;
}

mod private {
    pub trait Sealed {}
    impl<I> Sealed for crate::i2c::Bypass<I> {}
    impl<S> Sealed for crate::spi::SPI<S> {}
}
