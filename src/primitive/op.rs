//! Generic operations for `const` contexts.

pub mod binary;
pub mod unary;

use self::{binary::BinOp, unary::UnOp};

pub(self) use paste::item;

pub trait Operate = BinOp + UnOp;
pub trait Operable = Copy + Sized;

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