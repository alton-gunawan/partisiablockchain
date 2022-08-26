use std::collections::{BTreeMap, BTreeSet};

/// This trait adds the runtime type information needed to generate the [contract PBC ABI files](https://privacyblockchain.gitlab.io/language/rust-contract-sdk/abiv1.html).
///
/// * The [`__ty_name`](Self::__ty_name) method returns ordinary Rust names.
/// * The [`__ty_identifier`](Self::__ty_identifier) method recursively creates the PBC ABI byte-serialized type specification
///
/// Custom implementations should be rare, and throughly tested, as a malformed ABI might seriously
/// affect intercontract communication, and might even prevent initialization. Ensure validity with
/// respect to the [ABI specification](https://partisiablockchain.gitlab.io/documentation/abiv1.html).
///
/// Defines the ABI serialization format.
pub trait CreateTypeSpec: Sized {
    /// Produce the name of the implementing type.
    fn __ty_name() -> String;

    /// A unique identifier for this type. Generated at compile time for structs.
    fn __ty_identifier() -> String;

    /// Write the type spec for the type toi the given byte vector.
    /// See docs for [`CreateTypeSpec`].
    fn __ty_spec_write(w: &mut Vec<u8>, lut: &BTreeMap<String, u8>);
}

/// Implement the [`CreateTypeSpec`] trait for a 'simple' type given a type name and a type ordinal.
///
/// The input is n pairs of (type, literal).
///
/// The output is n implementations of [`CreateTypeSpec`] that simply write the type as a string
/// and fill the ordinal in the [`CreateTypeSpec::__ty_ordinal`] vector output.
macro_rules! impl_for_type {
    ($($type:ty, $val:literal)*) => {
        $(
            #[doc = "Implementation of the [`CreateTypeSpec`] trait for [`"]
            #[doc = stringify!($type)]
            #[doc = "`]."]
            impl CreateTypeSpec for $type {

                #[doc = concat!("Constant string `", stringify!($type), "`.")]
                fn __ty_name() -> String {
                    format!("{}", quote!($type).to_string())
                }

                #[doc = concat!("Ordinal is `", stringify!($val), "`, as defined in [ABI Spec](https://partisiablockchain.gitlab.io/documentation/abiv1.html).")]
                fn __ty_identifier() -> String {
                    Self::__ty_name()
                }

                fn __ty_spec_write( w: &mut Vec<u8>, _lut: &BTreeMap<String, u8>) {
                    w.push($val)
                }
            }
        )*
    }
}

// Implement the [`CreateTypeSpec`] trait for simple types.
//
// Byte values are taken from the [ABI spec](https://privacyblockchain.gitlab.io/language/rust-contract-sdk/abiv1.html). Due to macro_rules restrictions they can't be used as named constant.
impl_for_type!(
    u8,     0x01
    u16,    0x02
    u32,    0x03
    u64,    0x04
    u128,   0x05
    i8,     0x06
    i16,    0x07
    i32,    0x08
    i64,    0x09
    i128,   0x0a
    String, 0x0b
    bool,   0x0c
);

/// Implementation of the [`CreateTypeSpec`] trait for [`Vec<T>`] for any `T` that implements
/// [`CreateTypeSpec`].
impl<T: CreateTypeSpec> CreateTypeSpec for Vec<T> {
    /// Type name is constant string `Vec<T>`.
    fn __ty_name() -> String {
        format!("Vec<{}>", T::__ty_name())
    }

    /// Ordinal is `0x0e` followed by ordinal of `T`, as defined in [ABI Spec](https://partisiablockchain.gitlab.io/documentation/abiv1.html).
    fn __ty_identifier() -> String {
        format!("Vec<{}>", T::__ty_identifier())
    }

    fn __ty_spec_write(w: &mut Vec<u8>, lut: &BTreeMap<String, u8>) {
        // Vector is 0x0e followed by the spec for the parameter type
        w.push(0x0e);
        T::__ty_spec_write(w, lut);
    }
}

/// Implementation of the [`CreateTypeSpec`] trait for for [`BTreeMap<K, V>`]
/// for any K, V that implement [`CreateTypeSpec`].
impl<K: CreateTypeSpec, V: CreateTypeSpec> CreateTypeSpec for BTreeMap<K, V> {
    /// Type name is `BTreeMap<T>`.
    fn __ty_name() -> String {
        format!("BTreeMap<{}, {}>", K::__ty_name(), V::__ty_name())
    }

    /// Ordinal is `0x0f` followed by ordinals of `K` and `V`, as defined in [ABI Spec](https://partisiablockchain.gitlab.io/documentation/abiv1.html).
    fn __ty_identifier() -> String {
        format!(
            "BTreeMap<{}, {}>",
            K::__ty_identifier(),
            V::__ty_identifier()
        )
    }

    fn __ty_spec_write(w: &mut Vec<u8>, lut: &BTreeMap<String, u8>) {
        // BTreeMap is 0x0f followed by the spec for key type and then the value type
        w.push(0x0f);
        K::__ty_spec_write(w, lut);
        V::__ty_spec_write(w, lut);
    }
}

/// Implementation of the [`CreateTypeSpec`] trait for [`BTreeSet<T>`]
/// for any `T` that implements [`CreateTypeSpec`]
impl<V: CreateTypeSpec> CreateTypeSpec for BTreeSet<V> {
    /// Type name is `BTreeSet<T>`
    fn __ty_name() -> String {
        format!("BTreeSet<{}>", V::__ty_name())
    }

    /// Ordinal is `0x10` followed by ordinal of `T`, as defined in [ABI Spec](https://partisiablockchain.gitlab.io/documentation/abiv1.html).
    fn __ty_identifier() -> String {
        format!("BTreeSet<{}>", V::__ty_identifier())
    }

    fn __ty_spec_write(w: &mut Vec<u8>, lut: &BTreeMap<String, u8>) {
        // BTreeSet is 0x10 followed by the spec for the parameter type
        w.push(0x10);
        V::__ty_spec_write(w, lut);
    }
}

/// Implementation of the [`CreateTypeSpec`] trait for [`Option<T>`]
/// for any `T` that implements [`CreateTypeSpec`]
impl<T: CreateTypeSpec> CreateTypeSpec for Option<T> {
    /// Type name is `Option<T>`.
    fn __ty_name() -> String {
        format!("Option<{}>", T::__ty_name())
    }

    /// Ordinal is `0x12` followed by ordinal of `T`, as defined in [ABI Spec](https://partisiablockchain.gitlab.io/documentation/abiv1.html).
    fn __ty_identifier() -> String {
        format!("Option<{}>", T::__ty_identifier())
    }

    fn __ty_spec_write(w: &mut Vec<u8>, lut: &BTreeMap<String, u8>) {
        w.push(0x12);
        T::__ty_spec_write(w, lut);
    }
}

/// Implement [`CreateTypeSpec`] for an [`[u8;n]`] array type.
///
/// The ordinal is 0x11 followed by the length of the array
impl<const LEN: usize> CreateTypeSpec for [u8; LEN] {
    /// Type name is `[u8; LEN]`.
    fn __ty_name() -> String {
        format!("[u8; {}]", LEN)
    }

    fn __ty_identifier() -> String {
        Self::__ty_name()
    }

    /// Ordinal is `0x11` followed by byte repr of length `LEN`, as defined in [ABI Spec](https://partisiablockchain.gitlab.io/documentation/abiv1.html).
    fn __ty_spec_write(w: &mut Vec<u8>, _lut: &BTreeMap<String, u8>) {
        w.push(0x11);
        let length = u8::try_from(LEN)
            .ok()
            .filter(|&x| x <= 0x7F)
            .expect("ABI does not support byte arrays of sizes larger than 127.");
        w.push(length);
    }
}
