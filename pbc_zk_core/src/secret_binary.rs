use crate::SecretBinaryFixedSize;

/// Array implementation of [`SecretBinaryFixedSize`]
#[automatically_derived]
#[allow(unused, clippy::unused_unit)]
impl<const LEN: usize, ElementT: SecretBinaryFixedSize> SecretBinaryFixedSize for [ElementT; LEN] {
    const BITS: u32 = (LEN as u32) * <ElementT as SecretBinaryFixedSize>::BITS;
}

// Tuple implementations of [`SecretBinaryFixedSize`]
macro_rules! tuple_impls {
    ( $( $name:ident )* ) => {
        #[automatically_derived]
        #[allow(unused, clippy::unused_unit)]
        impl<$($name: SecretBinaryFixedSize),*> SecretBinaryFixedSize for ($($name,)*)
        {
            const BITS: u32 = $(<$name as SecretBinaryFixedSize>::BITS+)* 0;
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
