#![no_std]

pub mod spi;
use mpu9250_regs as regs;

use embedded_hal::blocking::delay::DelayMs;

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

/// `Transport` lets us generalize device configuration across both
/// SPI and I2C interfaces. It should *not* be used for higher-speed
/// data queries; those should be implemented manually, using the
/// best approach available for your peripheral.
trait Transport {
    type Error;
    fn mpu9250_read(&mut self, register: regs::mpu9250::Regs) -> Result<u8, Error<Self::Error>>;
    fn mpu9250_write<B: Copy + Into<u8>>(
        &mut self,
        register: regs::mpu9250::Regs,
        value: B,
    ) -> Result<(), Error<Self::Error>>;
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

    fn ak8963_read(&mut self, register: regs::ak8963::Regs) -> Result<u8, Error<Self::Error>>;
    fn ak8963_write<B: Copy + Into<u8>>(
        &mut self,
        register: regs::ak8963::Regs,
        value: B,
    ) -> Result<(), Error<Self::Error>>;
}

/// Perform common startup initialization for the device
fn begin<T: Transport>(
    transport: &mut T,
    delay: &mut dyn DelayMs<u8>,
) -> Result<(), Error<T::Error>> {
    use crate::regs::{
        ak8963::flags::*, ak8963::Regs as AK8963, mpu9250::flags::*, mpu9250::Regs as MPU9250,
    };

    // Enable the I2C interface, just so we can power-down the AK8963...
    transport.mpu9250_write(MPU9250::USER_CTRL, USER_CTRL::I2C_MST_EN.bits())?;
    transport.mpu9250_write(
        MPU9250::I2C_MST_CTRL,
        I2C_MST_CTRL::clock(I2C_MST_CLK::KHz400),
    )?;

    // Bring down both the AK8963 and the MPU9250
    transport.ak8963_write(AK8963::CNTL1, CNTL1::power_down())?;
    transport.mpu9250_write(MPU9250::PWR_MGMT_1, PWR_MGMT_1::reset())?;
    delay.delay_ms(10);

    // Re-enable the I2C interface.
    // Disable the I2C slave interface here, so that it doesn't think
    // we're talking to it as an I2C device.
    transport.mpu9250_write(
        MPU9250::USER_CTRL,
        (USER_CTRL::I2C_MST_EN | USER_CTRL::I2C_IF_DIS).bits(),
    )?;
    transport.mpu9250_write(
        MPU9250::I2C_MST_CTRL,
        I2C_MST_CTRL::clock(I2C_MST_CLK::KHz400),
    )?;

    // Soft-reset the AK8963
    transport.ak8963_write(AK8963::CNTL2, CNTL2::SRST.bits())?;

    // Set the clock source for the gyro
    transport.mpu9250_write(
        MPU9250::PWR_MGMT_1,
        PWR_MGMT_1::clock_select(PWR_MGMT_1_CLKSEL::AutoSelect),
    )?;

    // Sanity-check the WHO_AM_I values for both devices. By this point, we should be able
    // to address them.
    let who_am_i = transport.mpu9250_read(MPU9250::WHO_AM_I)?;
    if !crate::regs::mpu9250::VALID_WHO_AM_I.contains(&who_am_i) {
        return Err(Error::WhoAmI {
            expected: crate::regs::mpu9250::VALID_WHO_AM_I,
            actual: who_am_i,
        });
    }

    let who_am_i = transport.ak8963_read(AK8963::WIA)?;
    if !crate::regs::ak8963::VALID_WHO_AM_I.contains(&who_am_i) {
        return Err(Error::WhoAmI {
            expected: crate::regs::ak8963::VALID_WHO_AM_I,
            actual: who_am_i,
        });
    }
    Ok(())
}
