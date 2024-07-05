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
            impl const Cast<$target_out> for $target_type {
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

use std::primitive;

pub(self) use impl_castable;
pub(self) use impl_tests;

use crate::primitive::primitive_list;


#[const_trait]
/// A trait for casting between primitive types.
pub trait Cast<O> {
    /// Explicitly cast the value to the target type.
    /// 
    /// NOTE: This cast may result in truncation of bits.
    fn cast(self) -> O;
}


primitive_list!(impl_castable);

primitive_list!(impl_tests);