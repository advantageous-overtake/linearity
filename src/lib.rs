#![feature(const_trait_impl, associated_type_defaults, effects, trait_alias, const_ptr_as_ref, ptr_as_ref_unchecked)]
#![allow(incomplete_features, unused_macros)]
#![doc = include_str!("../README.md")]

use primitive::{
    cast::Cast,
    op::binary::{BitAnd, BitXor},
    Primitive,
};
pub use blanket::{PointerExt, PointerMutExt};
pub use filter::Filter;


mod filter;
mod blanket;
pub mod primitive;

/// Constant-accelerated trait for various operations commonly found in branchless programming.
#[const_trait]
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
        T: ~const Primitive,
        T: ~const BitXor<Operand = T, Output = T> + ~const BitAnd<Operand = T, Output = T>,

        Self: ~const Cast<T>,
        i8: ~const Cast<T>,
    {
        let select_mask: T = Filter::condition(target_dependence);
        let target_left: T = self.cast();

        let target_operand = target_left.xor(target_right);

        target_left.xor(
            target_operand.and(select_mask)
        )
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
        T: ~const Primitive,
        T: ~const BitAnd<Operand = T, Output = T>,

        Self: ~const Cast<T>,
        i8: ~const Cast<T>,
    {
        let select_mask: T = target_dependence.mask();
        let target_operand: T = self.cast();

        target_operand.and(select_mask)
    }
}

impl<T> const Linearity for T where T: Primitive {}

#[cfg(test)]
mod tests {
    use crate::primitive::primitive_list;
    use paste::item;
    use const_random::const_random as random;

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