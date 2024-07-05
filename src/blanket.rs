//! Blanket implementation for implementors of the [`super::Linearity`] trait and such.

use crate::Linearity;
use core::ops::Deref;

/// Extension trait for pointers.
pub trait PointerExt<T> {
    /// Selects between two pointers based on a predicate.
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because it involves raw pointer manipulation.
    /// 
    /// The caller must guarantee that the target pointers are valid.
    unsafe fn select(self, target_other: Self, target_predicate: bool) -> Self;

    /// Selects between two pointers and dereferences the selected pointer.
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because it involves raw pointer manipulation and reads from an arbitrary memory location.
    ///
    /// The caller must guarantee that the target pointers are valid.
    unsafe fn select_deref(self, target_other: Self, target_predicate: bool) -> T;
}

/// Extension trait for mutable pointers.
pub trait PointerMutExt<T>: PointerExt<T> {
    
}

impl<T> PointerExt<T> for *const T
where 
    T: Copy,
    usize: Linearity
{
    #[inline]
    unsafe fn select_deref(self, target_other: Self, target_predicate: bool) -> T
    where 
        T: Copy
    {
        *self.select(target_other, target_predicate).as_ref_unchecked()
    }
    
    #[inline]
    unsafe fn select(self, target_other: Self, target_dependence: bool) -> Self {
        let target_left = self as usize;
        let target_right = target_other as usize;

        let target_outcome = usize::select(target_left, target_right, target_dependence);

        target_outcome as Self
    }
}


impl<T> PointerExt<T> for *mut T
where 
    T: Copy,
    usize: Linearity
{
    #[inline]
    unsafe fn select_deref(self, target_other: Self, target_predicate: bool) -> T
    where 
        T: Copy
    {
        *self.select(target_other, target_predicate).as_ref_unchecked()
    }
    
    #[inline]
    unsafe fn select(self, target_other: Self, target_dependence: bool) -> Self {
        let target_left = self as usize;
        let target_right = target_other as usize;

        let target_outcome = usize::select(target_left, target_right, target_dependence);

        target_outcome as Self
    }
}

impl<T> PointerMutExt<T> for *mut T
where 
    *mut T: PointerExt<T>
{
    
}