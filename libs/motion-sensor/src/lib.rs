//! Interfaces for motion sensors

#![no_std]

mod triplet;
pub use triplet::Triplet;

/// An accelerometer reading
pub type Acc<V> = Triplet<V>;

/// An accelerometer
pub trait Accelerometer<V> {
    type Error;
    /// Query the accelerometer values
    fn accelerometer(&mut self) -> Result<Acc<V>, Self::Error>;
}

/// A gyroscope reading
pub type Gyro<V> = Triplet<V>;

/// A gyroscope
pub trait Gyroscope<V> {
    type Error;
    /// Query the gyroscope values
    fn gyroscope(&mut self) -> Result<Gyro<V>, Self::Error>;
}

/// A magnetometer reading
pub type Mag<V> = Triplet<V>;

/// A magnetometer
pub trait Magnetometer<V> {
    type Error;
    /// Query the magnetometer values
    fn magnetometer(&mut self) -> Result<Mag<V>, Self::Error>;
}

/// A combination of an accelerometer and a gyroscope
///
/// The default implementation simply queries the accelerometer, then queries the
/// gyroscope. Implementations that can perform a more efficient query should do
/// so.
pub trait DOF6<A, G>:
    Accelerometer<A> + Gyroscope<G, Error = <Self as Accelerometer<A>>::Error>
{
    fn dof6(&mut self) -> Result<(Acc<A>, Gyro<G>), <Self as Accelerometer<A>>::Error> {
        Ok((self.accelerometer()?, self.gyroscope()?))
    }
}

pub type MARGReadings<A, G, M> = (Acc<A>, Gyro<G>, Mag<M>);

/// All three of a accelerometer, magnetometer, and gyroscope
///
/// The default implementation queries for all three readings separately.
/// Consider providing a querying optimization if you're able to do so.
pub trait MARG<A, G, M>:
    Accelerometer<A>
    + Gyroscope<G, Error = <Self as Accelerometer<A>>::Error>
    + Magnetometer<M, Error = <Self as Accelerometer<A>>::Error>
{
    fn marg(&mut self) -> Result<MARGReadings<A, G, M>, <Self as Accelerometer<A>>::Error> {
        Ok((
            self.accelerometer()?,
            self.gyroscope()?,
            self.magnetometer()?,
        ))
    }
}
