//! See [`Filter`] for more information.

use crate::primitive::{cast::Cast, Primitive};

/// Represents a filter that can be either transparent or opaque.
/// 
/// *NOTE*: The behaviour of this type is value-dependant.
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    /// Allows values through.
    Transparent = 0b0000_0000,
    /// Does not allow values through.
    Opaque = 0b0000_0001,
}

impl Filter {
    /// Creates a new `Filter` instance based on the target condition.
    #[inline]
    pub const fn new<T>(target_cond: T) -> Self
    where 
        T: Into<bool>
    {
        match target_cond.into() {
            true => Self::Transparent,
            false => Self::Opaque,
        }
    }

    /// Returns the condition as a generic type `T`.
    #[inline]
    pub const fn condition<T>(target_cond: bool) -> T
    where
        T: ~const Primitive,
        i8: ~const Cast<T>,
    {
        Self::new(target_cond)
            .mask()
    }

    /// Returns the mask value as a generic type `T`.
    #[inline]
    pub const fn mask<T>(self) -> T
    where
        T: ~const Primitive,
        i8: ~const Cast<T>,
    {
        let mask = self as i8;
        
        mask
            /*
                If `Filter` is `Opaque` (1), then the mask value will be `-1`, which corresponds to all bits being set.
                If `Filter` is `Transparent` (0), then the mask value will be `0`, which corresponds to all bits being unset.
             */
            .wrapping_sub(1)
            .cast()
    }
}