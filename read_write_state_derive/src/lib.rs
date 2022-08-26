//! A crate for deriving `ReadWriteState`.
extern crate derive_commons;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use derive_commons::impl_read_write;

/// Implement `ReadWriteState` for the annotated struct.
#[proc_macro_derive(ReadWriteState)]
pub fn read_write(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_read_write(
        &ast,
        format_ident!("ReadWriteState"),
        format_ident!("state_read_from"),
        format_ident!("state_write_to"),
        Some(make_serialize_by_copy_constant),
    );

    // Return the generated impl
    gen.into()
}

/// Creator for the derived [`ReadWriteState::SERIALIZABLE_BY_COPY`] associated constant.
///
/// See documentation for [`ReadWriteState::SERIALIZABLE_BY_COPY`]; this derived constant is based
/// upon the value of `SERIALIZABLE_BY_COPY` for all composite fields, creating a constant
/// expression `and`ing them all together. The choice of operator is due to the contagious property
/// of `not SERIALIZABLE_BY_COPY`; e.g. if one component of the struct must be copied "manually",
/// the entire struct must be copied "manually".
///
/// Other conditions may occur in the constant expression, depending upon the circumstances.
fn make_serialize_by_copy_constant(
    supported_kind: &derive_commons::SupportedKind,
    trait_name: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let const_id = format_ident!("SERIALIZABLE_BY_COPY");
    let expression = match supported_kind {
        derive_commons::SupportedKind::StructWithNamedFields(fields) => {
            let fieldtypes: Vec<_> = fields.iter().map(derive_commons::field_to_type).collect();
            make_serialize_by_copy_constant_struct(&fieldtypes, trait_name, &const_id)
        }
        derive_commons::SupportedKind::DiscriminatedCstyleEnum(_discriminant_type, _variants) => {
            quote! { true }
        }
    };
    quote! {
        const #const_id : bool = #expression;
    }
}

fn make_serialize_by_copy_constant_struct(
    fieldtypes: &[proc_macro2::TokenStream],
    trait_name: &proc_macro2::Ident,
    const_id: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    // Construct core expression, of form: (if `S = struct { T_0, ..., T_n }`)
    //
    //     T_0 & T_1 & ... & T_n & true
    //
    // Note that the trailing `true` is used as "default" value in case of zero fields / empty types.
    let fields_serializable_by_copy =
        quote! { (#( <#fieldtypes as pbc_traits::#trait_name>::#const_id &)* true) };

    // Construct alignment expression, of form:
    //
    //    size_of<S>() modulo align_of<S>() == 0
    //
    // This check guarentees that the type is only SERIALIZABLE_BY_COPY if values can be packed
    // together in slices without any padding between.
    //
    // This is a temporary limitation, as we are currently not supporting SERIALIZABLE_BY_COPY in
    // padded structs.
    let aligned_at_size =
        quote! { (std::mem::size_of::<Self>().wrapping_rem(std::mem::align_of::<Self>()) == 0) };

    // Construct padding check expression, of form:
    //
    //    size_of<S>() == size_of<T_0>() + size_of<T_1>() + ... + size_of<T_n>() + 0
    //
    // Note that the trailing `0` is used as "default" value in case of zero fields / empty types.
    //
    // This check guarentees that the type is only SERIALIZABLE_BY_COPY if values contains no
    // padding bytes between fields.
    //
    // This is a temporary limitation, as we are currently not supporting SERIALIZABLE_BY_COPY in
    // padded structs.
    let bytes_used_by_fields = quote! { #( std::mem::size_of::<#fieldtypes>() +)* 0 };
    let tightly_packed = quote! { (std::mem::size_of::<Self>() == #bytes_used_by_fields ) };

    // Collect all expressions together.
    quote! { #fields_serializable_by_copy & #aligned_at_size & #tightly_packed }
}
