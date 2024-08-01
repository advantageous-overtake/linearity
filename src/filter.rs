/*
  linearity: A library for branchless programming
    Copyright (C) 2024  advantageous-overtake

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
    pub fn new<T>(target_cond: T) -> Self
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
    pub fn condition<T>(target_cond: bool) -> T
    where
        T: Primitive,
        i8: Cast<T>,
    {
        Self::new(target_cond)
            .mask()
    }

    /// Returns the mask value as a generic type `T`.
    #[inline]
    pub fn mask<T>(self) -> T
    where
        T: Primitive,
        i8: Cast<T>,
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