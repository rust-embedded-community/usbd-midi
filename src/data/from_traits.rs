//! Traits for type conversions.

/// Like from, but will conceptually overflow if the value is too big
/// this is useful from going from higher ranges to lower ranges
pub trait FromOverFlow<T>: Sized {
    /// Convert with overflowing.
    fn from_overflow(_: T) -> Self;
}

/// Like from, but will clamp the value to a maximum value
pub trait FromClamped<T>: Sized {
    /// Convert with clamping.
    fn from_clamped(_: T) -> Self;
}
