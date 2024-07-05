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
                #[const_trait]
                pub trait [< Bit $target_name:camel >]: Operable {
                    type Output = Self;

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
                impl const [< Bit $target_name:camel >] for $target_type {
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
            #[const_trait]
            pub trait UnOp:
                $(
                    [< Bit $target_name:camel >] +
                )+
                Operable
            {
                type Output = Self;
            }

            impl<T> const UnOp for T
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

unary_list!(unary);
unary_list!(impl_unary);

unary_list!(impl_supertrait);

/// Expands to a generic bound constricting the target type to a target unary operation.
/// 
/// *NOTE* This is an internal macro, it is not intended for use outside of this crate.
macro_rules! unary_constrict {
    (
        $target_op:ident as $target_type:ty
    ) => {
        $crate::primitive::op::item!(
            $target_type: ~const [< Bit $target_op:camel >]<Output = $target_type>,
        )
    };
}

/// Implements all constrict macros for unary operations.
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
                    ($target_type:ty) => {
                        $crate::primitive::op::unary::unary_constrict!($target_name as $target_type);
                    };
                }

                pub(crate) use [< $target_name:snake >];
            )+
        }
    };
}

unary_list!(impl_constrict_macros);

pub(self) use unary_constrict;

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
                use super::UnOp;
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