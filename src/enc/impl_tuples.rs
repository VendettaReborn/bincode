use super::{Encode, Encoder};
use crate::error::EncodeError;

macro_rules! impl_tuple {
    {$($index:tt : $name:tt),*} => {
        impl<$($name),*> Encode for ($($name,)*)
        where
            $($name: Encode),*
        {
            fn encode<EC: Encoder>(&self, encoder: &mut EC) -> Result<(), EncodeError> {
                $(self.$index.encode(encoder)?;)*
                Ok(())
            }
        }
    };
}

impl_tuple! {0: A}
impl_tuple! {0: A, 1: B}
impl_tuple! {0: A, 1: B, 2: C}
impl_tuple! {0: A, 1: B, 2: C, 3: D}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L, 12: M}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L, 12: M, 13: N}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L, 12: M, 13: N, 14: O}
impl_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L, 12: M, 13: N, 14: O, 15: P}
