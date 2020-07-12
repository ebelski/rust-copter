//! Interfaces for motion sensors

#![no_std]

mod triplet;
pub use triplet::Triplet;

/// The default scalar value type
type DefaultScalar = f32;

/// An accleration reading in Gs
pub type Gs<V = DefaultScalar> = Triplet<V>;

/// An accelerometer
pub trait Accelerometer<V = DefaultScalar> {
    type Error;
    /// Query the accelerometer values
    fn accelerometer(&mut self) -> Result<Gs<V>, Self::Error>;
}

/// A gyroscope reading in degrees per second
pub type DegPerSec<V = DefaultScalar> = Triplet<V>;

/// A gyroscope
pub trait Gyroscope<V = DefaultScalar> {
    type Error;
    /// Query the gyroscope values
    fn gyroscope(&mut self) -> Result<DegPerSec<V>, Self::Error>;
}

/// A magnetometer reading in uT
pub type MicroT<V = DefaultScalar> = Triplet<V>;

/// A magnetometer
pub trait Magnetometer<V = DefaultScalar> {
    type Error;
    /// Query the magnetometer values
    fn magnetometer(&mut self) -> Result<MicroT<V>, Self::Error>;
}

/// A combination of an accelerometer and a gyroscope
///
/// The default implementation simply queries the accelerometer, then queries the
/// gyroscope. Implementations that can perform a more efficient query should do
/// so.
pub trait DOF6<A = DefaultScalar, G = DefaultScalar>:
    Accelerometer<A> + Gyroscope<G, Error = <Self as Accelerometer<A>>::Error>
{
    fn dof6(&mut self) -> Result<(Gs<A>, DegPerSec<G>), <Self as Accelerometer<A>>::Error> {
        Ok((self.accelerometer()?, self.gyroscope()?))
    }
}

pub type MARGReadings<A = DefaultScalar, G = DefaultScalar, M = DefaultScalar> =
    (Gs<A>, DegPerSec<G>, MicroT<M>);

/// All three of a accelerometer, magnetometer, and gyroscope
///
/// The default implementation queries for all three readings separately.
/// Consider providing a querying optimization if you're able to do so.
pub trait MARG<A = DefaultScalar, G = DefaultScalar, M = DefaultScalar>:
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
