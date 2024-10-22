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

//! Abstraction over all primitive types.

pub mod op;
pub mod cast;

use cast::Cast;
use op::Operate;

use paste::item;


/// Returns the signless version of the primitive type.
/// 
/// Prepend the type identifier with either `u` or `i` to get the unsigned or signed version of the primitive type respectively.
/// 
/// Optionally allows to invoke the target macro with an additional token tree, which is expanded as:
/// 
/// ```ignore
/// $target_macro!($lead_tt as <...>);
/// ```
/// 
/// `<...>` being a placeholder for the tokens to be emitted.
macro_rules! integer_list {
    ($target_macro:path $(=> $lead_tt:tt)?) => {
        $target_macro!(
            $($lead_tt as)?
            8 16 32 64 128 size
        );
    };
}

/// Implements the primitive macros.
macro_rules! impl_primitive_macros {
    (
        $(
            $target_pospend:tt
        )*
    ) => {
        $crate::primitive::item! {
            /// A macro for generating a list of primitive types.
            macro_rules! primitive_list {
                ($target_macro:path => $lead_tt:tt) => {
                    $target_macro!(
                        $lead_tt as
                        $(
                            [< u $target_pospend >]
                            [< i $target_pospend >]
                        )*

                    );
                };
                ($target_macro:path) => {
                    $target_macro!(
                        $(
                            [< u $target_pospend >]
                            [< i $target_pospend >]
                        )*
                    );
                };
            }

            /// Expands to the signed variant of an unsigned primitive.
            macro_rules! signed {
                $(
                    ([< u $target_pospend >]) => { [< i $target_pospend >] };
                )*

                // Previous braces are exhaustive over available unsigned types.
                // So, we can safely assume that the input is signed.
                // The signed countervariant of a signed primitive is themselves.
                ($target_type:ty) => { $target_type };
            }

            /// Expands to the unsigned variant of a signed primitive.
            macro_rules! unsigned {
                $(
                    ([< i $target_pospend >]) => { [< u $target_pospend >] };
                )*

                // Previous expand options are exhaustive over available signed types.
                // So, we can safely assume that the input is unsigned.
                // The unsigned countervariant of a unsigned primitive is themselves.
                ($target_type:ty) => { $target_type };
            }

            /// Expands to a boolean literal which denotes whether the type is signed or not.
            macro_rules! is_signed {
                $(
                    ([< i $target_pospend >]) => { true };
                    ([< u $target_pospend >]) => { false };
                )*
            }

            /// A macro for generating a list of signed primitive types.
            #[allow(unused_macros)]
            macro_rules! signed_list {
                ($target_macro:path => $lead_tt:tt) => {
                    $target_macro!(
                        $lead_tt as
                        $(
                            [< i $target_pospend >]
                        )*
                    );
                };
                ($target_macro:path) => {
                    $target_macro!(
                        $(
                            [< i $target_pospend >]
                        )*
                    );
                };
            }

            /// A macro for generating a list of unsigned primitive types.
            #[allow(unused_macros)]
            macro_rules! unsigned_list {
                ($target_macro:path => $lead_tt:tt) => {
                    $target_macro!(
                        $lead_tt as
                        $(
                            [< u $target_pospend >]
                        )*
                    );
                };
                ($target_macro:path) => {
                    $target_macro!(
                        $(
                            [< u $target_pospend >]
                        )*
                    );
                };
            }
        }

        pub(crate) use primitive_list;

        pub(crate) use signed;
        pub(crate) use unsigned;
        pub(crate) use is_signed;

        #[allow(unused_imports)]
        pub(crate) use signed_list;
        #[allow(unused_imports)]
        pub(crate) use unsigned_list;
    }
}

/// Implements the primitive trait for the target primitive types.
macro_rules! impl_primitive {
    (
        $target_trait:ty as
        $(
            $target_type:ident
        )+
    ) => {
        $(
            impl $crate::private::Sealed for $target_type {}

            impl $target_trait for $target_type {
                type Signed = $crate::primitive::signed!($target_type);
                type Unsigned = $crate::primitive::unsigned!($target_type);

                const SIGNED: bool =  $crate::primitive::is_signed!($target_type);

                const MIN: Self = <$target_type>::MIN;
                const MAX: Self = <$target_type>::MAX;
                const BITS: u8 = <$target_type>::BITS as _;
            }
        )+
    };
    (
        $(
            $target_pospend:tt
        )*
    ) => {
       $crate::primitive::impl_primitive_macros! {
           $(
               $target_pospend
           )*
       }

       $crate::primitive::primitive_list!($crate::primitive::impl_primitive => Primitive);
    };
}

/// Implement a sealed marker trair for a set of types.
macro_rules! marker {
    (
        $target_trait:ident as $($target_type:ident)* 
    ) => {
        $(
            impl $target_trait for $target_type {}
        )*
    };
}

pub(self) use impl_primitive;
pub(self) use impl_primitive_macros;
pub(self) use integer_list;

integer_list!(impl_primitive);



/// Marker trait for signed types.
pub trait Signed: crate::private::Sealed {}


/// Marker trait for unsigned types.
pub trait Unsigned: crate::private::Sealed {}

signed_list!(marker => Signed);
unsigned_list!(marker => Unsigned);

/// Primitive trait for all primitive types.
pub trait Primitive: Operate + crate::private::Sealed {
    /// The signed version of the primitive type.
    type Signed: Primitive + Cast<Self>;
    /// The unsigned version of the primitive type.
    type Unsigned: Primitive + Cast<Self>;

    /// Whether this numeric primitive is singed or not.
    const SIGNED: bool;

    /// The minimum value of the primitive type.
    const MIN: Self;
    /// The maximum value of the primitive type.
    const MAX: Self;
    /// The number of bits in the primitive type.
    const BITS: u8;
}
