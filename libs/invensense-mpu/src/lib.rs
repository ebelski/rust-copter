//! Implementation of the `motion-sensor` interfaces for an Invensense MPU9250

#![no_std]

pub mod i2c;
pub mod spi;

/// Re-export the registers under a different name
pub mod regs {
    pub use mpu9250_regs::{
        ak8963::flags::*, ak8963::Regs as AK8963, ak8963::I2C_ADDRESS as AK8963_I2C_ADDRESS,
        mpu9250::flags::*, mpu9250::Regs as MPU9250, mpu9250::I2C_ADDRESS as MPU9250_I2C_ADDRESS,
    };
}

use core::fmt::Debug;
use embedded_hal::blocking::delay::DelayMs;
use motion_sensor::Triplet;

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

/// An Invensense MPU
///
/// The `MPU` struct can be used to query readings from a physical
/// MPU.
pub struct MPU<T> {
    transport: T,
    handle: Handle,
}

type Sensitivity = Triplet<f64>;

impl<T> MPU<T>
where
    T: Transport,
{
    fn new(transport: T, config: &Config, sensitivity: &Sensitivity) -> Self {
        MPU {
            transport,
            handle: Handle::new(&config, sensitivity),
        }
    }
}

impl<T> MPU<T> {
    fn scale_gyro(&self, raw: Triplet<i16>) -> Triplet<f64> {
        raw.map(|raw| self.handle.gyro_resolution * f64::from(raw))
    }

    fn scale_acc(&self, raw: Triplet<i16>) -> Triplet<f64> {
        const GRAVITY: f64 = 9.807;
        raw.map(|raw| self.handle.acc_resolution * GRAVITY * f64::from(raw))
    }

    fn scale_mag(&self, raw: Triplet<i16>) -> Triplet<f64> {
        raw.map(|raw| self.handle.mag_resolution * f64::from(raw)) * self.handle.mag_sensitivity
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

/// Holds controller-side state of the MPU9250
pub struct Handle {
    gyro_resolution: f64,
    acc_resolution: f64,
    mag_resolution: f64,
    mag_sensitivity: Triplet<f64>,
}

impl Handle {
    fn new(config: &Config, mag_sensitivity: &Sensitivity) -> Handle {
        use regs::*;
        Handle {
            gyro_resolution: match config.gyro_scale {
                GYRO_FS_SEL::DPS250 => 250.0,
                GYRO_FS_SEL::DPS500 => 500.0,
                GYRO_FS_SEL::DPS1000 => 1000.0,
                GYRO_FS_SEL::DPS2000 => 2000.0,
            } / 32768.0,
            acc_resolution: match config.accel_scale {
                ACCEL_FS_SEL::G2 => 2.0,
                ACCEL_FS_SEL::G4 => 4.0,
                ACCEL_FS_SEL::G8 => 8.0,
                ACCEL_FS_SEL::G16 => 16.0,
            } / 32768.0,
            mag_resolution: if config.mag_scale.output.is_empty() {
                10. * 4912. / 8190.
            } else {
                10. * 4912. / 32760.0
            },
            mag_sensitivity: *mag_sensitivity,
        }
    }
}

/// Scaling and sampling rates
///
/// See the default values for all members to understand what a default
/// `Config` looks like.
///
/// ```
/// use invensense_mpu::Config;
/// use mpu9250_regs::mpu9250::flags::*;
///
/// let mut config = Config::default();
/// config.gyro_scale = GYRO_FS_SEL::DPS500;
/// config.dlpf = DLPF::_4;
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Config {
    /// Gyroscope scale
    pub gyro_scale: regs::GYRO_FS_SEL,
    /// Gyroscope sampling frequency selection
    pub fchoice: regs::FCHOICE,
    /// Digital low pass filter configuration
    pub dlpf: regs::DLPF,
    /// Accelerometer scaling
    pub accel_scale: regs::ACCEL_FS_SEL,
    /// Acceleromter data rate (DLPF) selection
    pub accel_rate: regs::ACCEL_CONFIG_2,
    /// Magnetometer resolution
    pub mag_scale: regs::CNTL1,
    /// Configures the sample rate, `Sample_Rate`, as
    ///
    /// ```text
    /// Sample_Rate = Internal_Sample_Rate / (1 + sample_rate_divider)
    /// ```
    ///
    /// The setting takes effect only when `FCHOICE` is `DLPF`
    pub sample_rate_divider: u8,
}

impl Config {
    /// Writes the configuration to the connected MPU
    fn apply<T: Transport>(&self, transport: &mut T) -> Result<(), Error<T::Error>> {
        use regs::*;
        transport.mpu9250_write(
            MPU9250::GYRO_CONFIG,
            GYRO_CONFIG {
                full_scale: self.gyro_scale,
                fchoice: self.fchoice,
                ..Default::default()
            },
        )?;
        transport.mpu9250_write(MPU9250::CONFIG, self.dlpf)?;
        transport.mpu9250_write(
            MPU9250::ACCEL_CONFIG,
            ACCEL_CONFIG {
                full_scale: self.accel_scale,
                ..Default::default()
            },
        )?;
        transport.mpu9250_write(MPU9250::ACCEL_CONFIG_2, self.accel_rate)?;
        transport.mpu9250_write(MPU9250::SMPLRT_DIV, self.sample_rate_divider)?;
        transport.ak8963_write(AK8963::CNTL1, self.mag_scale)?;
        Ok(())
    }
}

/// Acquires magnetometer sensitivity values
///
/// Assumes that we can already talk to the AK8963. When this function returns,
/// The AK8963 will be in the mode it was in when received.
fn mag_sensitivity<T: Transport>(
    transport: &mut T,
    delay: &mut dyn DelayMs<u8>,
) -> Result<Triplet<f64>, Error<T::Error>> {
    use regs::*;
    let cntl1 = transport.ak8963_read(AK8963::CNTL1)?;
    transport.ak8963_write(
        AK8963::CNTL1,
        CNTL1 {
            mode: CNTL1_MODE::FUSE_ROM_ACCESS,
            ..Default::default()
        },
    )?;
    delay.delay_ms(50);
    let bias = Triplet {
        x: transport.ak8963_read(AK8963::ASAX)?,
        y: transport.ak8963_read(AK8963::ASAY)?,
        z: transport.ak8963_read(AK8963::ASAZ)?,
    };
    let sensitivity = bias.map(|bias| f64::from(bias - 128) / 256f64 + 1f64);
    transport.ak8963_write(AK8963::CNTL1, cntl1)?;
    delay.delay_ms(20);
    Ok(sensitivity)
}
