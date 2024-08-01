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

//! Generic operations for branchless programming.

pub mod binary;
pub mod unary;

use self::{binary::BinOp, unary::UnOp};

pub(self) use paste::item;

/// A trait representing a type that can be operated on.
pub trait Operate: BinOp + UnOp {}
/// A trait that represents a cheaply copyable type.
pub trait Operable: Copy + Sized {}

impl<T> Operate for T where T: BinOp + UnOp {}
impl<T> Operable for T where T: Copy + Sized {}


macro_rules! operate_list {
    ($target_macro:path $(=> $lead_tt:tt)?) => {
        $target_macro!(
            $($lead_tt as)?
            Binary {
                And as &
                Or as |
                Xor as ^
                Shl try <<
                Shr try >>
            }
            Unary {
                Not as !
            }
        );
    };
}



macro_rules! impl_list_macros {
    (
        Binary {
            $(
                $target_name:ident $target_mode:ident $target_symbol:tt
            )+
        }
        Unary {
            $(
                $target_name_unary:ident as $target_symbol_unary:tt
            )+
        }
    ) => {
        macro_rules! binary_list {
            ($target_macro:path) => {
                $target_macro!(
                    $($target_name as $target_symbol)+
                );
            };
        }

        macro_rules! unary_list {
            ($target_macro:path) => {
                $target_macro!(
                    $($target_name_unary as $target_symbol_unary)+
                );
            };
        }

        pub(self) use binary_list;
        pub(self) use unary_list;
    };
}

operate_list!(impl_list_macros);

pub(self) use operate_list;