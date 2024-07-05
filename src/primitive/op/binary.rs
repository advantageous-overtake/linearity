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
                #[const_trait]
                pub trait [< Bit $target_name:camel >]: Operable {
                    type Operand = Self;
                    type Output = Self;

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
                impl const [< Bit $target_name:camel >] for $target_type {
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
            #[const_trait]
            /// Supertrait for all binary operations.
            pub trait BinOp:
                $(
                    [< Bit $target_name:camel >]<> +
                )+
                Operable
            {
                type Operand = Self;
                type Output = Self;
            }

            impl<T> const BinOp for T
            where T:
                $(
                    [< Bit $target_name:camel >] +
                )+
                Operable
            {

            }
        }
    };
}

binary_list!(binary);
binary_list!(impl_binary);

binary_list!(impl_supertrait);

/// Expands to a generic bound constricting the target type to a target binary operation.
/// 
/// *NOTE* This is an internal macro, it is not intended for use outside of this crate.
macro_rules! binary_constrict {
    (
        $target_type:ty as $target_name:ident
    ) => {
        $crate::primitive::op::item! {
            $target_type: ~const [< Bit $target_name:camel >]<Operand = $target_type, Output = $target_type>,
        }
    };
}

#[allow(unused_imports)]
pub(self) use binary_constrict;

/// Implements all constrict macros for binary operations.
macro_rules! impl_constrict_macros {
    (
        $(
            $target_name:ident as $_:tt
        )+
    ) => {
        $crate::primitive::op::item! {
            $(
                #[doc = "Constricts the target type to the `" $target_name "` operation."]
                #[macro_export]
                macro_rules! [< $target_name:snake >] {
                    ($target_type:tt) => {
                        $crate::primitive::op::binary::binary_constrict!($target_type as $target_name);
                    };
                }

                #[allow(unused_imports)]
                pub(crate) use [< $target_name:snake >];
            )+
        }
    };
}

binary_list!(impl_constrict_macros);


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