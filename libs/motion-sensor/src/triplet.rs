//! Triplet helper type for three-axis readings

/// A reading `T` on three axes
///
/// The axis orientations are dependent on the implementation. Consult
/// your implementation's documentation for more information.
///
/// `Triplet<T>` converts to and from both `[T; 3]` and `(T, T, T)`.
/// By convention, the zeroth element is X; the first Y; the second Z.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Triplet<T> {
    /// Reading from the X axis
    pub x: T,
    /// Reading from the Y axis
    pub y: T,
    /// Reading from the Z axix
    pub z: T,
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
