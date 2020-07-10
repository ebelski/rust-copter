//! SPI interface for an MPU9250

use crate::{Error, Transport};
use embedded_hal::{blocking::delay::DelayMs, blocking::spi::Transfer};
use motion_sensor::{Accelerometer, Gyroscope, Magnetometer, Triplet};
use mpu9250_regs as regs;

use core::fmt::Debug;

const fn read(address: regs::mpu9250::Regs) -> u16 {
    ((address as u16) | (1 << 7)) << 8
}

const fn write(address: regs::mpu9250::Regs, value: u8) -> u16 {
    ((address as u16) << 8) | (value as u16)
}

impl<S> Transport for SPI<S>
where
    S: Transfer<u16>,
    S::Error: Debug,
{
    type Error = S::Error;
    fn mpu9250_read(&mut self, register: regs::mpu9250::Regs) -> Result<u8, Error<Self::Error>> {
        let mut buffer = [read(register)];
        self.0
            .transfer(&mut buffer)
            .map(|buffer| {
                let value = (buffer[0] & 0xFF) as u8;
                log::trace!("READ {:?} => {:#04X}", register, value);
                value
            })
            .map_err(|err| {
                log::error!("READ {:?}: {:?}", register, err);
                err.into()
            })
    }
    fn mpu9250_write<B: Copy + Into<u8>>(
        &mut self,
        register: regs::mpu9250::Regs,
        value: B,
    ) -> Result<(), Error<Self::Error>> {
        let value = value.into();
        let mut buffer = [write(register, value)];
        self.0
            .transfer(&mut buffer)
            .map(|_| {
                log::trace!("WRITE {:?} <= {:#04X}", register, value);
            })
            .map_err(|err| {
                log::error!("WRITE {:?}: {:?}", register, err);
                err.into()
            })
    }
    fn ak8963_read(&mut self, register: regs::ak8963::Regs) -> Result<u8, Error<Self::Error>> {
        ak8963_read(&mut self.0, register)
            .map(|value| {
                log::trace!("READ {:?} => {:#04X}", register, value);
                value
            })
            .map_err(|err| {
                log::error!("READ {:?}: {:?}", register, err);
                err.into()
            })
    }
    fn ak8963_write<B: Copy + Into<u8>>(
        &mut self,
        register: regs::ak8963::Regs,
        value: B,
    ) -> Result<(), Error<Self::Error>> {
        ak8963_write(&mut self.0, register, value.into())
            .map(|_| {
                log::trace!("WRITE {:?} <= {:#04X}", register, value.into());
            })
            .map_err(|err| {
                log::error!("WRITE {:?}: {:?}", register, err);
                err.into()
            })
    }
}

pub struct SPI<S>(S);

impl<S> SPI<S>
where
    S: Transfer<u16>,
    S::Error: Debug,
{
    pub fn new(spi: S, delay: &mut dyn DelayMs<u8>) -> Result<Self, Error<S::Error>> {
        use regs::ak8963::flags::{CNTL1, CNTL1_MODE, CNTL1_OUTPUT};
        use regs::ak8963::Regs;
        use regs::ak8963::I2C_ADDRESS as AK8963_I2C_ADDRESS;
        use regs::mpu9250::flags::{
            ACCEL_CONFIG, ACCEL_FS_SEL, FCHOICE, GYRO_CONFIG, GYRO_FS_SEL, I2C_SLVX_CTRL,
            I2C_SLVX_CTRL_FLAGS, PWR_MGMT_1, USER_CTRL,
        };
        use regs::mpu9250::Regs::*;

        let mut spi = SPI(spi);

        spi.mpu9250_write(PWR_MGMT_1, PWR_MGMT_1::H_RESET.bits())?;
        delay.delay_ms(100);
        // Select the best clock
        spi.mpu9250_write(PWR_MGMT_1, 0x01)?;
        // Enable all accelerometer and gyro axes
        spi.mpu9250_write(PWR_MGMT_2, 0x00)?;
        // Reset interrupt state
        spi.mpu9250_write(INT_ENABLE, 0x00)?;
        // Set gyro range
        //
        // Setting fchoice might also require a write to config. See the docs
        // for details.
        spi.mpu9250_write(
            GYRO_CONFIG,
            GYRO_CONFIG {
                // TODO these should be configurable
                full_scale: GYRO_FS_SEL::DPS250,
                fchoice: FCHOICE::DLPF,
                ..Default::default()
            },
        )?;
        spi.mpu9250_write(CONFIG, 0)?;
        // Set accel range
        spi.mpu9250_write(
            ACCEL_CONFIG,
            ACCEL_CONFIG {
                full_scale: ACCEL_FS_SEL::G2,
                ..Default::default()
            },
        )?;
        // Set accel data rate
        // TODO this should be configurable
        spi.mpu9250_write(ACCEL_CONFIG_2, 0)?;
        // TODO set data rate SMPLRT_DIV

        // Check the user control register to see if we're properly configured.
        // If we're OK, we shouldn't write to the USER_CTRL register with the reset
        // bit.
        let user_ctrl: USER_CTRL = USER_CTRL::from_bits_truncate(spi.mpu9250_read(USER_CTRL)?);
        if user_ctrl != (USER_CTRL::I2C_MST_RST | USER_CTRL::I2C_IF_DIS) {
            // Not in the setting we need. Attempt a reset with a command to enter the correct
            // state.
            let user_ctrl: USER_CTRL =
                USER_CTRL::I2C_MST_RST | USER_CTRL::I2C_IF_DIS | USER_CTRL::I2C_MST_EN;
            spi.mpu9250_write(USER_CTRL, user_ctrl.bits())?;
            delay.delay_ms(100);
        }

        // TODO get magnetomter biases, calibration values?

        // Power-down the magnetometer, then bring it back up for
        //
        // - 100Hz sampling
        // - 16-bit resolution
        spi.ak8963_write(Regs::CNTL1, 0)?;
        delay.delay_ms(100);
        spi.ak8963_write(
            Regs::CNTL1,
            CNTL1 {
                mode: CNTL1_MODE::CONTINUOUS_2,
                output: Default::default(),
            },
        )?;
        delay.delay_ms(10);

        // 400KHz I2C clock
        spi.mpu9250_write(I2C_MST_CTRL, 0x0D)?;
        delay.delay_ms(10);

        // Configure magnetometer sampling
        spi.mpu9250_write(I2C_SLV0_ADDR, AK8963_I2C_ADDRESS | (1 << 7))?;
        spi.mpu9250_write(I2C_SLV0_REG, Regs::HXL as u8)?;
        // TODO figure out the grouping and byte swapping; the docs aren't clear...
        spi.mpu9250_write(
            I2C_SLV0_CTRL,
            I2C_SLVX_CTRL {
                flags: I2C_SLVX_CTRL_FLAGS::EN,
                length: 7,
            },
        )?;
        delay.delay_ms(100);

        Ok(spi)
    }

    /// Query the MPU9250's `WHO_AM_I` register
    pub fn mpu9250_who_am_i(&mut self) -> Result<u8, Error<S::Error>> {
        self.mpu9250_read(regs::mpu9250::Regs::WHO_AM_I)
    }

    /// Query the AK8963's `WHO_AM_I` register
    pub fn ak8963_who_am_i(&mut self) -> Result<u8, Error<S::Error>> {
        self.ak8963_read(regs::ak8963::Regs::WIA)
    }
}

impl<S> Accelerometer for SPI<S>
where
    S: Transfer<u16>,
{
    type Value = u16;
    type Error = Error<S::Error>;
    fn accelerometer(&mut self) -> Result<Triplet<Self::Value>, Self::Error> {
        const COMMANDS: [u16; 6] = [
            read(regs::mpu9250::Regs::ACCEL_XOUT_H),
            read(regs::mpu9250::Regs::ACCEL_XOUT_L),
            read(regs::mpu9250::Regs::ACCEL_YOUT_H),
            read(regs::mpu9250::Regs::ACCEL_YOUT_L),
            read(regs::mpu9250::Regs::ACCEL_ZOUT_H),
            read(regs::mpu9250::Regs::ACCEL_ZOUT_L),
        ];
        let mut buffer = COMMANDS;
        self.0.transfer(&mut buffer)?;
        Ok(Triplet {
            x: (buffer[0] << 8) | (buffer[1] & 0xFF),
            y: (buffer[2] << 8) | (buffer[3] & 0xFF),
            z: (buffer[4] << 8) | (buffer[5] & 0xFF),
        })
    }
}

impl<S> Gyroscope for SPI<S>
where
    S: Transfer<u16>,
{
    type Value = u16;
    type Error = Error<S::Error>;
    fn gyroscope(&mut self) -> Result<Triplet<Self::Value>, Self::Error> {
        const COMMANDS: [u16; 6] = [
            read(regs::mpu9250::Regs::GYRO_XOUT_H),
            read(regs::mpu9250::Regs::GYRO_XOUT_L),
            read(regs::mpu9250::Regs::GYRO_YOUT_H),
            read(regs::mpu9250::Regs::GYRO_YOUT_L),
            read(regs::mpu9250::Regs::GYRO_ZOUT_H),
            read(regs::mpu9250::Regs::GYRO_ZOUT_L),
        ];
        let mut buffer = COMMANDS;
        self.0.transfer(&mut buffer)?;
        Ok(Triplet {
            x: (buffer[0] << 8) | (buffer[1] & 0xFF),
            y: (buffer[2] << 8) | (buffer[3] & 0xFF),
            z: (buffer[4] << 8) | (buffer[5] & 0xFF),
        })
    }
}

impl<S> Magnetometer for SPI<S>
where
    S: Transfer<u16>,
{
    type Value = u16;
    type Error = Error<S::Error>;
    fn magnetometer(&mut self) -> Result<Triplet<Self::Value>, Self::Error> {
        const COMMANDS: [u16; 6] = [
            read(regs::mpu9250::Regs::EXT_SENS_DATA_00),
            read(regs::mpu9250::Regs::EXT_SENS_DATA_01),
            read(regs::mpu9250::Regs::EXT_SENS_DATA_02),
            read(regs::mpu9250::Regs::EXT_SENS_DATA_03),
            read(regs::mpu9250::Regs::EXT_SENS_DATA_04),
            read(regs::mpu9250::Regs::EXT_SENS_DATA_05),
        ];
        let mut buffer = COMMANDS;
        self.0.transfer(&mut buffer)?;
        Ok(Triplet {
            x: (buffer[1] << 8) | (buffer[0] & 0xFF),
            y: (buffer[3] << 8) | (buffer[2] & 0xFF),
            z: (buffer[5] << 8) | (buffer[4] & 0xFF),
        })
    }
}

/// Read from the AK8963's register identified by `register`
fn ak8963_read<SPI: Transfer<u16>>(
    spi: &mut SPI,
    register: regs::ak8963::Regs,
) -> Result<u8, Error<SPI::Error>> {
    use regs::ak8963::I2C_ADDRESS;
    use regs::mpu9250::{flags::I2C_SLV4_CTRL, Regs::*};

    spi.transfer(&mut [write(I2C_SLV4_ADDR, I2C_ADDRESS | (1 << 7))])?;
    spi.transfer(&mut [write(I2C_SLV4_REG, register as u8)])?;
    spi.transfer(&mut [write(I2C_SLV4_CTRL, I2C_SLV4_CTRL::I2C_SLV4_EN.bits())])?;
    ak8963_wait_done(spi, 10_000, register, None)?;

    let mut buffer = [read(I2C_SLV4_DI)];
    spi.transfer(&mut buffer)?;
    Ok((buffer[0] & 0xFF) as u8)
}

/// Write's `value` to the AK8963's `register`
fn ak8963_write<SPI: Transfer<u16>>(
    spi: &mut SPI,
    register: regs::ak8963::Regs,
    value: u8,
) -> Result<(), Error<SPI::Error>> {
    use regs::ak8963::I2C_ADDRESS;
    use regs::mpu9250::{flags::I2C_SLV4_CTRL, Regs::*};

    spi.transfer(&mut [write(I2C_SLV4_ADDR, I2C_ADDRESS | (1 << 7))])?;
    spi.transfer(&mut [write(I2C_SLV4_REG, register as u8)])?;
    spi.transfer(&mut [write(I2C_SLV4_DO, value)])?;
    spi.transfer(&mut [write(I2C_SLV4_CTRL, I2C_SLV4_CTRL::I2C_SLV4_EN.bits())])?;
    ak8963_wait_done(spi, 10_000, register, Some(value))?;
    Ok(())
}

/// Wait `max_attempts` for the indication that the I2C transation is complete
fn ak8963_wait_done<SPI: Transfer<u16>>(
    spi: &mut SPI,
    max_attempts: u16,
    register: regs::ak8963::Regs,
    value: Option<u8>,
) -> Result<(), Error<SPI::Error>> {
    use regs::mpu9250::{flags::I2C_MST_STATUS, Regs::*};
    for _ in 0..max_attempts {
        let mut buffer = [read(I2C_MST_STATUS)];
        spi.transfer(&mut buffer)?;
        let status = I2C_MST_STATUS::from_bits_truncate((buffer[0] & 0xFF) as u8);
        if status.contains(I2C_MST_STATUS::I2C_SLV4_DONE) {
            return Ok(());
        } else if status.contains(I2C_MST_STATUS::I2C_SLV4_NACK) {
            return Err(Error::Nack);
        } else if status.contains(I2C_MST_STATUS::I2C_LOST_ARB) {
            return Err(Error::LostArbitration);
        }
    }
    Err(Error::Timeout {
        attempts: max_attempts,
        register,
        value,
    })
}
