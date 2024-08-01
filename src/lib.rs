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

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub use blanket::{PointerExt, PointerMutExt};
pub use filter::Filter;
use primitive::{
    cast::Cast,
    op::binary::{BitAnd, BitXor},
    Primitive,
};

mod blanket;
mod filter;
pub mod primitive;

pub mod prelude;
/// Constant-accelerated trait for various operations commonly found in branchless programming.
pub trait Linearity: Primitive {
    /// Performs a selection operation between `self` and `target_right` based on the value of `target_dependence`.
    ///
    /// This function takes three arguments:
    /// * `target_right` - The target value to be selected.
    /// * `target_dependence` - A boolean value indicating whether to select `self` or `target_right`.
    ///
    /// The function returns the selected value of type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use linearity::Linearity;
    ///
    /// let value = 5;
    /// let target_right = 10;
    /// let target_dependence = true;
    ///
    /// let selected_value = value.select(target_right, target_dependence);
    ///
    /// assert_eq!(selected_value, target_right);
    /// ```
    #[inline]
    fn select<T>(self, target_right: T, target_dependence: bool) -> T
    where
        T: Primitive,
        T: BitXor<Operand = T, Output = T> + BitAnd<Operand = T, Output = T>,

        Self: Cast<T>,
        i8: Cast<T>,
    {
        let select_mask: T = Filter::condition(target_dependence);
        let target_left: T = self.cast();

        let target_operand = target_left.xor(target_right);

        target_left.xor(target_operand.and(select_mask))
    }

    /// Performs a filter operation between `self` and `target_dependence`.
    ///
    /// The function returns the filtered value of type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use linearity::Linearity;
    /// use linearity::Filter;
    ///
    /// let value = 5;
    /// let target_dependence = Filter::Opaque;
    ///
    /// let filtered_value: i32 = value.filter(target_dependence);
    ///
    /// assert_eq!(filtered_value, 0);
    /// ```
    ///
    #[inline]
    fn filter<T>(self, target_dependence: Filter) -> T
    where
        T: Primitive,
        T: BitAnd<Operand = T, Output = T>,

        Self: Cast<T>,
        i8: Cast<T>,
    {
        let select_mask: T = target_dependence.mask();
        let target_operand: T = self.cast();

        target_operand.and(select_mask)
    }
}

impl<T> Linearity for T where T: Primitive {}

#[cfg(test)]
mod tests {
    use crate::primitive::primitive_list;
    use const_random::const_random as random;
    use paste::item;

    macro_rules! impl_select {
        (
            $(
                $target_ty:ident
            )+
        ) => {
            item!(
                $(
                    #[test]
                    fn [< impl_ select_ $target_ty:snake >] () {
                        use super::Linearity;

                        let target_left: $target_ty = random!($target_ty);
                        let target_right: $target_ty = random!($target_ty);

                        let target_dependence: bool = random!(u8) == random!(u8);


                        let target_expect: $target_ty = if target_dependence {
                            target_right
                        } else {
                            target_left
                        };

                        let target_value: $target_ty = target_left.select(target_right, target_dependence);

                        assert_eq!(target_expect, target_value);
                    }
                )+
            );
        };
    }

    macro_rules! impl_filter {
        (
            $(
                $target_ty:ident
            )+
        ) => {
            item!(
                $(
                    #[test]
                    fn [< impl_ filter_ $target_ty:snake >] () {
                        use super::Linearity;
                        use super::filter::Filter;

                        let target_left: $target_ty = random!($target_ty);
                        let target_dependence: Filter = Filter::new(random!(u8) == random!(u8));

                        let target_expect: $target_ty = match target_dependence {
                            Filter::Opaque => 0,
                            Filter::Transparent => target_left,
                        };

                        let target_value: $target_ty = target_left.filter(target_dependence);

                        assert_eq!(target_expect, target_value);
                    }
                )+
            );
        };
    }

    primitive_list!(impl_select);

    primitive_list!(impl_filter);
}
