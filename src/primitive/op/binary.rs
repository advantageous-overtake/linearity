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

//! All binary operation traits and implementations.

use super::{binary_list, Operable};

/// Declares all binary operation traits.
macro_rules! binary {
    (
        $(
            $target_name:ident as $target_symbol:tt
        )+
    ) => {
        $crate::primitive::op::item! {
            $(
                #[doc = "The `" $target_name "` operation."]
                pub trait [< Bit $target_name:camel >]: Operable {
                    /// The type of the right operand.
                    type Operand;
                    /// The output type.
                    type Output;

                    #[doc = "Performs the `" $target_name "` operation."]
                    fn [< $target_name:snake >](self, target_right: Self::Operand) -> Self::Output;
                }
            )+
        }
    };
    () => {};
}

/// Implements all binary operation traits for a primitive.
macro_rules! impl_binary_for {
    (
        ($target_name:ident as $target_symbol:tt) as
        $(
            $target_type:ty
        )+
    ) => {
        $crate::primitive::op::item! {
            $(
                impl [< Bit $target_name:camel >] for $target_type {
                    type Operand = Self;
                    type Output = Self;

                    #[inline]
                    fn [< $target_name:snake >](self, target_right: Self::Operand) -> Self::Output {
                        self $target_symbol target_right
                    }
                }
            )+
        }
    };
}

/// Implements all binary operation traits for all primitive types.
macro_rules! impl_binary {
    (
        $(
            $target_name:ident as $target_op:tt
        )+
    ) => {
        $(
            $crate::primitive::primitive_list!(impl_binary_for => ($target_name as $target_op));
        )+
    };
}

macro_rules! impl_supertrait {
    (
        $(
            $target_name:ident as $_l:tt
        )+
    ) => {
        $crate::primitive::op::item! {
            /// Supertrait for all binary operations.
            pub trait BinOp:
                $(
                    [< Bit $target_name:camel >]<> +
                )+
                Operable
            {
                /// The type of the left operand.
                type Operand;
                /// The type of the output.
                type Output;
            }

            impl<T> BinOp for T
            where T:
                $(
                    [< Bit $target_name:camel >] +
                )+
                Operable
            {
                type Operand = T;
                type Output = T;
            }
        }
    };
}

binary_list!(binary);
binary_list!(impl_binary);

binary_list!(impl_supertrait);

macro_rules! impl_tests {
    (
        ($target_type:ty as $target_operator:tt) as
        $(
            $target_integer:ty
        )+

        $(,)?
    ) => {
        $crate::primitive::item! {
            $(
                // NOTE: We check both for panic-behavior and correctness.
                #[test]
                fn [< binop_ $target_type:snake _impl_ $target_integer:snake >]() {
                    use std::{mem::discriminant, panic::catch_unwind};

                    let target_left: $target_integer = random!($target_integer);
                    let target_right: $target_integer = random!($target_integer);

                    let target_expect: Result<$target_integer, _> = catch_unwind(|| {
                        target_left $target_operator target_right
                    });

                    let target_value: Result<$target_integer, _> = catch_unwind(|| {
                        target_left . [< $target_type:snake >] (target_right)
                    });

                    match (target_expect, target_value) {
                        (Ok(expect), Ok(value)) => assert_eq!(expect, value),
                        (target_expect, target_value) => assert_eq!(discriminant(&target_expect), discriminant(&target_value))
                    }
                }
            )+
        }
    };
    (
        $(
            $target_name:ident as $target_operator:tt
        )+
    ) => {
        $crate::primitive::item! {
            #[cfg(test)]
            mod tests {
                use super::{
                    $(
                        [< Bit $target_name:camel >],
                    )+
                };

                use const_random::const_random as random;
                

                $(
                    $crate::primitive::primitive_list!($crate::primitive::op::binary::impl_tests => ($target_name as $target_operator));
                )+
            }
        }
    };
}

pub(self) use impl_tests;

binary_list!(impl_tests);