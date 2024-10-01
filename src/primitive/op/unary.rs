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

//! All unary operation traits and implementations.

use super::{unary_list, Operable};

/// Declares all unary operation traits.
macro_rules! unary {
    (
        $(
            $target_name:ident as $target_symbol:tt
        )+
    ) => {
        $crate::primitive::op::item! {
            $(
                #[doc = "The `" $target_name "` operation."]
                pub trait [< Bit $target_name:camel >]: Operable {
                    /// The output type.
                    type Output;

                    #[doc = "Performs the `" $target_name "` operation."]
                    fn [< $target_name:snake >](self) -> Self::Output;
                }
            )+
        }
    };
    () => {};
}

/// Implements all unary operation traits for a primitive.
macro_rules! impl_unary_for {
    (
        ($target_name:ident as $target_symbol:tt) as
        $(
            $target_type:ty
        )+
    ) => {
        $crate::primitive::op::item! {
            $(
                impl [< Bit $target_name:camel >] for $target_type {
                    type Output = $target_type;

                    #[inline]
                    fn [< $target_name:snake >](self) -> Self::Output {
                        $target_symbol self
                    }
                }
            )+
        }
    };
}

/// Implements all unary operation traits for all primitive types.
macro_rules! impl_unary {
    (
        $(
            $target_name:ident as $target_op:tt
        )+
    ) => {
        $(
            $crate::primitive::primitive_list!(impl_unary_for => ($target_name as $target_op));
        )+
    };
}

macro_rules! impl_supertrait {
    (
        $(
            $target_name:ident as $_:tt
        )+
    ) => {
        $crate::primitive::op::item! {
            /// A supertrait for all unary operations.
            pub trait UnOp:
                $(
                    [< Bit $target_name:camel >] +
                )+
                Operable
            {
                /// The output type for all underlying unary operations.
                type Output;
            }

            impl<T> UnOp for T
            where T:
                $(
                    [< Bit $target_name:camel >] +
                )+
                Operable
            {
                type Output = T;    
            }
        }
    };
}

unary_list!(unary);
unary_list!(impl_unary);

unary_list!(impl_supertrait);

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
                #[test]
                fn [< unop_ $target_type:snake _impl_ $target_integer:snake >]() {
                    let target_left: $target_integer = random!($target_integer);

                    let target_expect: $target_integer = $target_operator target_left;

                    let target_value: $target_integer = target_left . [< $target_type:snake >] ();

                    assert_eq!(target_expect, target_value);
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
                    $crate::primitive::primitive_list!($crate::primitive::op::unary::impl_tests => ($target_name as $target_operator));
                )+
            }
        }
    };
}

pub(self) use impl_tests;

unary_list!(impl_tests);