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

//! Module for blanket implementations and miscenalleous traits.

use crate::Linearity;

/// Extension trait for pointers.
pub trait PointerExt<T> {
    /// Selects between two pointers based on a predicate.
    ///
    /// # Safety
    ///
    /// Safety constrains are as per [`core::ptr::read`] for both inputs.
    unsafe fn select(self, target_other: Self, target_predicate: bool) -> Self;

    /// Selects between two pointers and dereferences the selected pointer.
    ///
    /// # Safety
    ///
    /// Safety constrains are as per [`core::ptr::read`] for both inputs.
    unsafe fn select_deref(self, target_other: Self, target_predicate: bool) -> T;
}

/// Extension trait for mutable pointers.
pub trait PointerMutExt<T>: PointerExt<T> {
    
}

impl<T> PointerExt<T> for *const T
where
    T: Copy,
    usize: Linearity,
{
    #[inline]
    unsafe fn select_deref(self, target_other: Self, target_predicate: bool) -> T {
        core::ptr::read(self.select(target_other, target_predicate))
    }

    #[inline]
    unsafe fn select(self, target_other: Self, target_dependence: bool) -> Self {
        let target_left = self as usize;
        let target_right = target_other as usize;

        let target_outcome = usize::select(target_left, target_right, target_dependence);

        target_outcome as Self
    }
}

impl<T> PointerExt<T> for *mut T {
    #[inline]
    unsafe fn select_deref(self, target_other: Self, target_predicate: bool) -> T {
        core::ptr::read(self.select(target_other, target_predicate))
    }

    #[inline]
    unsafe fn select(self, target_other: Self, target_dependence: bool) -> Self {
        let target_left = self as usize;
        let target_right = target_other as usize;

        let target_outcome =
            <usize as Linearity>::select(target_left, target_right, target_dependence);

        target_outcome as Self
    }
}

impl<T> PointerMutExt<T> for *mut T where *mut T: PointerExt<T> {}
