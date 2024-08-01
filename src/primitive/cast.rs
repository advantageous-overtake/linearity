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

//! Facilities for casting between primitive types in a generic fashion.
//! 
//! See [`Cast`] for more information.

macro_rules! impl_castable {
    (
        $target_type:ty as
        $(
            $target_out:ty
        )+

        $(,)?
    ) => {
        $(
            impl Cast<$target_out> for $target_type {
                #[inline]
                fn cast(self) -> $target_out {
                    self as $target_out
                }
            }
        )+
    };
    (
        $(
            $target_type:ty    
        )+

        $(,)?
    ) => {
        $(
            $crate::primitive::primitive_list!($crate::primitive::cast::impl_castable => $target_type);
        )+
    };
}

macro_rules! impl_tests {
    (
        $target_type:ty as
        $(
            $target_out:ty
        )+

        $(,)?
    ) => {
        $crate::primitive::item! {
            $(
                #[test]
                fn [< cast_ $target_type:snake _as_ $target_out:snake _ max >]() {
                    let target = <$target_type>::MAX;
                    let result: $target_out = target.cast();

                    assert_eq!(result, target as $target_out);
                }

                #[test]
                fn [< cast_ $target_type:snake _as_ $target_out:snake _ min >]() {
                    let target = <$target_type>::MIN;
                    let result: $target_out = target.cast();

                    assert_eq!(result, target as $target_out);
                }
            )+
        }
    };
    (
        $(
            $target_type:ty    
        )+

        $(,)?
    ) => {
        #[cfg(test)]
        mod tests {
            use super::Cast;

            $(
                $crate::primitive::primitive_list!($crate::primitive::cast::impl_tests => $target_type);
            )+
        }
    };
}

pub(self) use impl_castable;
pub(self) use impl_tests;

use crate::primitive::primitive_list;


/// A trait for casting between primitive types.
pub trait Cast<O> {
    /// Explicitly cast the value to the target type.
    /// 
    /// NOTE: This cast may result in truncation of bits.
    fn cast(self) -> O;
}


primitive_list!(impl_castable);

primitive_list!(impl_tests);