//! Interfaces for motion sensors

#![no_std]

mod triplet;
pub use triplet::Triplet;

pub trait Accelerometer {
    type Error;
    type Value;
    fn accelerometer(&mut self) -> Result<Triplet<Self::Value>, Self::Error>;
}

pub trait Gyroscope {
    type Error;
    type Value;
    fn gyroscope(&mut self) -> Result<Triplet<Self::Value>, Self::Error>;
}

pub trait Magnetometer {
    type Error;
    type Value;
    fn magnetometer(&mut self) -> Result<Triplet<Self::Value>, Self::Error>;
}

pub struct DOF6Readings<T> {
    pub accel: Triplet<T>,
    pub gyro: Triplet<T>,
}

pub trait DOF6:
    Accelerometer
    + Gyroscope<Value = <Self as Accelerometer>::Value, Error = <Self as Accelerometer>::Error>
{
    fn dof6(
        &mut self,
    ) -> Result<DOF6Readings<<Self as Accelerometer>::Value>, <Self as Accelerometer>::Error> {
        Ok(DOF6Readings {
            accel: self.accelerometer()?,
            gyro: self.gyroscope()?,
        })
    }
}

pub struct MARGReadings<T> {
    pub accel: Triplet<T>,
    pub gyro: Triplet<T>,
    pub mag: Triplet<T>,
}

pub trait MARG:
    Accelerometer
    + Gyroscope<Value = <Self as Accelerometer>::Value, Error = <Self as Accelerometer>::Error>
    + Magnetometer<Value = <Self as Accelerometer>::Value, Error = <Self as Accelerometer>::Error>
{
    fn marg(
        &mut self,
    ) -> Result<MARGReadings<<Self as Accelerometer>::Value>, <Self as Accelerometer>::Error> {
        Ok(MARGReadings {
            accel: self.accelerometer()?,
            gyro: self.gyroscope()?,
            mag: self.magnetometer()?,
        })
    }
}
