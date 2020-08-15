//! Interfaces for motion sensors
//!
//! Implementations of these traits must return values that are described by the
//! unit aliases. See the return types for more details.
//!
//! # Features
//!
//! Enable the `"use-serde"` flags to define an enum for motion sensor readings. The
//! enum can be serialized and deserialized.

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

/// A tuple of accelerometer, gyroscope, and magnetometer readings
///
/// See [`MARG`](trait.MARG.html) for more information.
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

/// Types that are exposed only when the "use-serde" feature is on
#[cfg(feature = "use-serde")]
mod ser_de {
    use super::{DefaultScalar, DegPerSec, Gs, MicroT};
    use serde::{Deserialize, Serialize};

    /// A motion sensor reading, with a tag that describes
    /// the measurement
    ///
    /// `Reading` can be serialized and deserialized with `serde`.
    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Reading<A = DefaultScalar, G = DefaultScalar, M = DefaultScalar> {
        /// Accelerometer reading
        Accelerometer(Gs<A>),
        /// Gyroscope reading
        Gyroscope(DegPerSec<G>),
        /// Magnetometer reading
        Magnetometer(MicroT<M>),
    }
}

#[cfg(feature = "use-serde")]
pub use ser_de::*;
