//! Triplet helper type for three-axis readings

#[cfg(feature = "use-serde")]
use serde::{Deserialize, Serialize};

/// A reading `T` on three axes
///
/// The axis orientations are dependent on the implementation. Consult
/// your implementation's documentation for more information.
///
/// `Triplet<T>` converts to and from both `[T; 3]` and `(T, T, T)`.
/// By convention, the zeroth element is X; the first Y; the second Z.
/// It can also be treated as a slice of three elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Triplet<T> {
    /// Reading from the X axis
    pub x: T,
    /// Reading from the Y axis
    pub y: T,
    /// Reading from the Z axix
    pub z: T,
}

impl<T> Triplet<T> {
    /// Performs an element by element operation `f` from each triplet,
    /// creating a new triplet
    pub fn scalar<F: Fn(T, T) -> T>(self, other: Triplet<T>, f: F) -> Triplet<T> {
        Triplet {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
            z: f(self.z, other.z),
        }
    }

    /// Applies `f` to each value in the triplet, creating a new triplet
    pub fn map<X, F: Fn(T) -> X>(self, f: F) -> Triplet<X> {
        Triplet {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    /// Returns the triplet as a slice
    ///
    /// Coordinates are ordered as `[x, y, z]`.
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(&self.x as *const T, 3) }
    }

    /// Returns the triplet as a mutable slice
    ///
    /// Coordinates are ordered as `[x, y, z]`.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(&mut self.x as *mut T, 3) }
    }
}

impl<T> From<[T; 3]> for Triplet<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Triplet { x, y, z }
    }
}

impl<T> From<Triplet<T>> for [T; 3] {
    fn from(Triplet { x, y, z }: Triplet<T>) -> Self {
        [x, y, z]
    }
}

impl<T> From<(T, T, T)> for Triplet<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Triplet { x, y, z }
    }
}

impl<T> From<Triplet<T>> for (T, T, T) {
    fn from(Triplet { x, y, z }: Triplet<T>) -> Self {
        (x, y, z)
    }
}

impl<T> core::ops::Mul for Triplet<T>
where
    T: core::ops::Mul<Output = T>,
{
    type Output = Triplet<T>;

    fn mul(self, other: Triplet<T>) -> Triplet<T> {
        self.scalar(other, core::ops::Mul::mul)
    }
}

impl<T> core::ops::Add for Triplet<T>
where
    T: core::ops::Add<Output = T>,
{
    type Output = Triplet<T>;

    fn add(self, other: Triplet<T>) -> Triplet<T> {
        self.scalar(other, core::ops::Add::add)
    }
}

impl<T> core::ops::Sub for Triplet<T>
where
    T: core::ops::Sub<Output = T>,
{
    type Output = Triplet<T>;

    fn sub(self, other: Triplet<T>) -> Triplet<T> {
        self.scalar(other, core::ops::Sub::sub)
    }
}

impl<T> core::ops::Div for Triplet<T>
where
    T: core::ops::Div<Output = T>,
{
    type Output = Triplet<T>;

    fn div(self, other: Triplet<T>) -> Triplet<T> {
        self.scalar(other, core::ops::Div::div)
    }
}

impl<T> core::fmt::Display for Triplet<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Triplet;

    #[test]
    fn as_slice() {
        let t = Triplet { x: 1, y: 2, z: 3 };
        assert_eq!(t.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn as_mut_slice() {
        let mut t = Triplet::from((1, 2, 3));
        t.as_mut_slice()[1] = 42;
        assert_eq!(t, Triplet::from([1, 42, 3]));
    }
}
