#![no_std]

pub mod spi;
use mpu9250_regs as regs;

use core::fmt::Debug;

/// Device errors that could occur during initialization or communication
#[derive(Debug)]
pub enum Error<P> {
    LostArbitration,
    Nack,
    Timeout {
        attempts: u16,
        register: regs::ak8963::Regs,
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
        self.transport.mpu9250_read(regs::mpu9250::Regs::WHO_AM_I)
    }

    /// Query the AK8963's `WHO_AM_I` register
    pub fn ak8963_who_am_i(&mut self) -> Result<u8, Error<T::Error>> {
        self.transport.ak8963_read(regs::ak8963::Regs::WIA)
    }
}

/// `Transport` lets us generalize device configuration across both
/// SPI and I2C interfaces. It should *not* be used for higher-speed
/// data queries; those should be implemented manually, using the
/// best approach available for your peripheral.
pub trait Transport: private::Sealed {
    type Error;

    /// Read a register from the MPU9250
    fn mpu9250_read(&mut self, register: regs::mpu9250::Regs) -> Result<u8, Error<Self::Error>>;
    /// Write a value to an MPU9250 register
    fn mpu9250_write<B: Copy + Into<u8>>(
        &mut self,
        register: regs::mpu9250::Regs,
        value: B,
    ) -> Result<(), Error<Self::Error>>;
    /// Modify an MPU9250 register
    ///
    /// By default, this performs a read then a write. If the supplied function
    /// returns `None`, the implementation aborts the write operation, and returns
    /// `Ok(())`.
    fn mpu9250_modify<F: FnOnce(u8) -> Option<B>, B: Copy + Into<u8>>(
        &mut self,
        register: regs::mpu9250::Regs,
        func: F,
    ) -> Result<(), Error<Self::Error>> {
        let value = self.mpu9250_read(register)?;
        match func(value) {
            None => Ok(()),
            Some(value) => self.mpu9250_write(register, value.into()),
        }
    }
    /// Read an AK8963 register
    fn ak8963_read(&mut self, register: regs::ak8963::Regs) -> Result<u8, Error<Self::Error>>;
    /// Write an AK8963 register
    fn ak8963_write<B: Copy + Into<u8>>(
        &mut self,
        register: regs::ak8963::Regs,
        value: B,
    ) -> Result<(), Error<Self::Error>>;
}

mod private {
    pub trait Sealed {}
    impl<S> Sealed for crate::spi::SPI<S> {}
}
