//! I2C interface
//!
//! Our current interface sets the MPU for I2C bypass mode. We directly talk to the on-board
//! magnetometer with our I2C controller. User is responsible for setting an appropriate I2C
//! clock speed.

use crate::{regs::*, Error, Handle, Transport, MPU};
use core::convert::TryInto;
use embedded_hal::blocking::{delay::DelayMs, i2c};
use motion_sensor::{
    Accelerometer, DOF6Readings, Gyroscope, MARGReadings, Magnetometer, Triplet, DOF6, MARG,
};

/// Bypass I2C mode
///
/// We directly talk to the AK8963 by disabling the MPU's I2C controller
pub struct Bypass<I>(I);

impl<I, E> Transport for Bypass<I>
where
    I: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    type Error = E;
    fn mpu9250_read(&mut self, register: MPU9250) -> Result<u8, Error<Self::Error>> {
        let mut out = [0; 1];
        self.0
            .write_read(MPU9250_I2C_ADDRESS, &[register as u8], &mut out)?;
        Ok(out[0])
    }
    fn mpu9250_write<B: Copy + Into<u8>>(
        &mut self,
        register: MPU9250,
        value: B,
    ) -> Result<(), Error<Self::Error>> {
        let buffer = [register as u8, value.into()];
        self.0.write(MPU9250_I2C_ADDRESS, &buffer)?;
        Ok(())
    }
    fn ak8963_read(&mut self, register: AK8963) -> Result<u8, Error<Self::Error>> {
        let mut out = [0; 1];
        self.0
            .write_read(AK8963_I2C_ADDRESS, &[register as u8], &mut out)?;
        Ok(out[0])
    }
    fn ak8963_write<B: Copy + Into<u8>>(
        &mut self,
        register: AK8963,
        value: B,
    ) -> Result<(), Error<Self::Error>> {
        let buffer = [register as u8, value.into()];
        self.0.write(AK8963_I2C_ADDRESS, &buffer)?;
        Ok(())
    }
}

/// Create a new MPU that uses I2C bypass
pub fn new<I, E>(i2c: I, delay: &mut dyn DelayMs<u8>) -> Result<MPU<Bypass<I>>, Error<E>>
where
    I: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    let mut i2c = Bypass(i2c);

    // Reset the MPU9250
    i2c.mpu9250_write(MPU9250::PWR_MGMT_1, PWR_MGMT_1::reset())?;
    delay.delay_ms(10);

    // Disable the I2C master interface by zeroing user control
    i2c.mpu9250_write(MPU9250::USER_CTRL, USER_CTRL::default())?;

    // Enable I2C bypass
    //
    // After this, we may call the ak8963 transport methods. They rely on this being set.
    i2c.mpu9250_write(MPU9250::INT_PIN_CFG, INT_PIN_CFG::BYPASS_EN)?;

    // Power-down the AK8963
    i2c.ak8963_write(AK8963::CNTL1, CNTL1::power_down())?;
    delay.delay_ms(10);

    // Soft-reset the AK8963
    i2c.ak8963_write(AK8963::CNTL2, CNTL2::SRST.bits())?;

    // Set the gyro clock source
    i2c.mpu9250_write(
        MPU9250::PWR_MGMT_1,
        PWR_MGMT_1::clock_select(PWR_MGMT_1_CLKSEL::AutoSelect),
    )?;

    // Sanity-check the WHO_AM_I values for both devices. By this point, we should be able
    // to address them.
    let who_am_i = i2c.mpu9250_read(MPU9250::WHO_AM_I)?;
    if !mpu9250_regs::mpu9250::VALID_WHO_AM_I.contains(&who_am_i) {
        return Err(Error::WhoAmI {
            expected: mpu9250_regs::mpu9250::VALID_WHO_AM_I,
            actual: who_am_i,
        });
    }

    let who_am_i = i2c.ak8963_read(AK8963::WIA)?;
    if !mpu9250_regs::ak8963::VALID_WHO_AM_I.contains(&who_am_i) {
        return Err(Error::WhoAmI {
            expected: mpu9250_regs::ak8963::VALID_WHO_AM_I,
            actual: who_am_i,
        });
    }

    // Set the AK8963 to continuous sampling
    i2c.ak8963_write(
        AK8963::CNTL1,
        CNTL1 {
            mode: CNTL1_MODE::CONTINUOUS_2,
            ..Default::default()
        },
    )?;

    Ok(MPU::new(i2c))
}

/// Release the I2C driver along with the driver handler for re-creating the
/// device
pub fn release<I>(mpu: MPU<Bypass<I>>) -> (I, Handle) {
    (mpu.transport.0, mpu.handle)
}

/// Reconstruct an MPU from an I2C peripheral and its handle
///
/// You're responsible for making sure that the handle matches to
/// the I2C peripheral it was originally associated with. Otherwise,
/// we associated the wrong state with a physical MPU.
pub fn from_handle<I>(i2c: I, handle: Handle) -> MPU<Bypass<I>>
where
    I: i2c::WriteRead + i2c::Write,
{
    MPU {
        transport: Bypass(i2c),
        handle,
    }
}

impl<I> Accelerometer for MPU<Bypass<I>>
where
    I: i2c::WriteRead,
{
    type Value = i16;
    type Error = I::Error;

    fn accelerometer(&mut self) -> Result<Triplet<Self::Value>, Self::Error> {
        let mut buffer = [0; 6];
        self.transport.0.write_read(
            MPU9250_I2C_ADDRESS,
            &[MPU9250::ACCEL_XOUT_H as u8],
            &mut buffer,
        )?;
        Ok(Triplet {
            x: i16::from_be_bytes(buffer[0..2].try_into().unwrap()),
            y: i16::from_be_bytes(buffer[2..4].try_into().unwrap()),
            z: i16::from_be_bytes(buffer[4..6].try_into().unwrap()),
        })
    }
}

impl<I> Gyroscope for MPU<Bypass<I>>
where
    I: i2c::WriteRead,
{
    type Value = i16;
    type Error = I::Error;

    fn gyroscope(&mut self) -> Result<Triplet<Self::Value>, Self::Error> {
        let mut buffer = [0; 6];
        self.transport.0.write_read(
            MPU9250_I2C_ADDRESS,
            &[MPU9250::GYRO_XOUT_H as u8],
            &mut buffer,
        )?;
        Ok(Triplet {
            x: i16::from_be_bytes(buffer[0..2].try_into().unwrap()),
            y: i16::from_be_bytes(buffer[2..4].try_into().unwrap()),
            z: i16::from_be_bytes(buffer[4..6].try_into().unwrap()),
        })
    }
}

impl<I> Magnetometer for MPU<Bypass<I>>
where
    I: i2c::WriteRead,
{
    type Value = i16;
    type Error = I::Error;

    fn magnetometer(&mut self) -> Result<Triplet<Self::Value>, Self::Error> {
        // Need to read 7 bytes here
        //
        // We need to touch ST2 in order to reset the magnetomter readings.
        let mut buffer = [0; 7];
        self.transport
            .0
            .write_read(AK8963_I2C_ADDRESS, &[AK8963::HXL as u8], &mut buffer)?;
        Ok(Triplet {
            x: i16::from_le_bytes(buffer[0..2].try_into().unwrap()),
            y: i16::from_le_bytes(buffer[2..4].try_into().unwrap()),
            z: i16::from_le_bytes(buffer[4..6].try_into().unwrap()),
        })
    }
}

impl<I> DOF6 for MPU<Bypass<I>>
where
    I: i2c::WriteRead,
{
    fn dof6(
        &mut self,
    ) -> Result<DOF6Readings<<Self as Accelerometer>::Value>, <Self as Accelerometer>::Error> {
        // Read through the temperature values to acquire the 6DOF readings in one I2C transaction
        let mut buffer = [0; 14];
        self.transport.0.write_read(
            MPU9250_I2C_ADDRESS,
            &[MPU9250::ACCEL_XOUT_H as u8],
            &mut buffer,
        )?;
        Ok(DOF6Readings {
            accel: Triplet {
                x: i16::from_be_bytes(buffer[0..2].try_into().unwrap()),
                y: i16::from_be_bytes(buffer[2..4].try_into().unwrap()),
                z: i16::from_be_bytes(buffer[4..6].try_into().unwrap()),
            }, // buffer[6..8] is temperature...
            gyro: Triplet {
                x: i16::from_be_bytes(buffer[8..10].try_into().unwrap()),
                y: i16::from_be_bytes(buffer[10..12].try_into().unwrap()),
                z: i16::from_be_bytes(buffer[12..14].try_into().unwrap()),
            },
        })
    }
}

impl<I> MARG for MPU<Bypass<I>>
where
    I: i2c::WriteRead,
{
    fn marg(
        &mut self,
    ) -> Result<MARGReadings<<Self as Accelerometer>::Value>, <Self as Accelerometer>::Error> {
        let DOF6Readings { accel, gyro } = self.dof6()?;
        let mag = self.magnetometer()?;
        Ok(MARGReadings { accel, gyro, mag })
    }
}
