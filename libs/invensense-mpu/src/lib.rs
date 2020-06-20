#![no_std]

pub mod spi;
use mpu9250_regs as regs;

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
}

impl<P> From<P> for Error<P> {
    fn from(err: P) -> Self {
        Error::Peripheral(err)
    }
}

/// `Transport` lets us generalize device configuration across both
/// SPI and I2C interfaces. It should *not* be used for higher-speed
/// data queries; those should be implemented manually, using the
/// best approach available for your peripheral.
trait Transport {
    type Error;
    fn mpu9250_read(&mut self, register: regs::mpu9250::Regs) -> Result<u8, Error<Self::Error>>;
    fn mpu9250_write<B: Into<u8>>(
        &mut self,
        register: regs::mpu9250::Regs,
        value: B,
    ) -> Result<(), Error<Self::Error>>;
    fn mpu9250_modify<F: FnOnce(u8) -> Option<B>, B: Into<u8>>(
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

    fn ak8963_read(&mut self, register: regs::ak8963::Regs) -> Result<u8, Error<Self::Error>>;
    fn ak8963_write<B: Into<u8>>(
        &mut self,
        register: regs::ak8963::Regs,
        value: B,
    ) -> Result<(), Error<Self::Error>>;
}
