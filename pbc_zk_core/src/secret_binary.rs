use crate::SecretBinaryFixedSize;
use std::io::Read;
use std::io::Write;

macro_rules! tuple_impls {
    ( $( $name:ident )* ) => {
        #[automatically_derived]
        #[allow(unused, clippy::unused_unit)]
        impl<$($name: SecretBinaryFixedSize),*> SecretBinaryFixedSize for ($($name,)*)
        {
            const BITS: u32 = $(<$name>::BITS+)* 0;
        }
    };
}

tuple_impls! {}
tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }
