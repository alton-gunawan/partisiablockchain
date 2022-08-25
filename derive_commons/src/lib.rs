//! Internal Partisia Blockchain SDK for procedural macro utility.
//!
//! Common functions used in the procedural macros in the SDK.
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse_quote::parse;
use syn::{Data, Expr, Field, Fields, Ident, Type};

/// Extracts a fields identifier
///
/// # Arguments
/// * `field` Field to extract identifier from.
pub fn field_to_name(field: &Field) -> Ident {
    field.ident.clone().unwrap()
}

/// Type for creating a new SERIALIZABLE_BY_COPY const field for the [`ReadWriteState`] trait.
///
/// Can assume that `Self` will refer to the implementing struct.
///
/// # Arguments
/// * First - Enum differentiating the kind of the derive type.
/// * Second - The trait name
type SerializableByCopyConstFieldCreator = fn(&SupportedKind, &Ident) -> TokenStream;

/// Implement a named trait that has read and a write methods.
/// The signature matches `ReadWriteRPC` and `ReadWriteState`.
///
/// # Arguments
/// * `ast` - A Abstract Syntax Tree of the struct calling the procedural macro.
/// * `trait_name` - Identifier of the trait to derive for
/// * `read_method` - Identifier of the trait read method
/// * `write_method` - Identifier of the trait write method
/// * `serializable_by_copy_creator` - If `Some` it must create a [`TokenStream`] for creating a new const field.
pub fn impl_read_write(
    ast: &syn::DeriveInput,
    trait_name: Ident,
    read_method: Ident,
    write_method: Ident,
    serializable_by_copy_creator: Option<SerializableByCopyConstFieldCreator>,
) -> proc_macro2::TokenStream {
    // Extract basic data
    let name = &ast.ident;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // Select (somewhat hygienic) names for internal types
    let type_read = format_ident!("Read{}", name);
    let type_write = format_ident!("Write{}", name);

    // Check that our choice was actually hygienic
    let names: std::collections::HashMap<&Ident, &'static str> = ast
        .generics
        .type_params()
        .map(|x| (&x.ident, "Type"))
        .chain(
            ast.generics
                .lifetimes()
                .map(|x| (&x.lifetime.ident, "Lifetime")),
        )
        .chain(
            ast.generics
                .const_params()
                .map(|x| (&x.ident, "Const generic")),
        )
        .collect();

    for generated_type_name in [&type_read, &type_write] {
        if let Some(kind) = names.get(generated_type_name) {
            panic!(
                "{} name {} collides with generated type name.",
                kind, generated_type_name
            );
        }
    }

    // Compute method logic
    let supported_kind = get_kind_data(&ast.data);
    let (read_logic, write_logic) = match &supported_kind {
        SupportedKind::StructWithNamedFields(fields) => {
            let fieldnames: Vec<_> = fields.iter().map(field_to_name).collect();
            let fieldtypes: Vec<_> = fields.iter().map(field_to_type).collect();

            make_read_and_write_logic_struct(&fieldnames, &fieldtypes, &read_method, &write_method)
        }
        SupportedKind::DiscriminatedCstyleEnum(discriminant_type, variants) => {
            let variant_names: Vec<_> = variants.iter().map(|x| x.0).collect();
            let variant_exprs: Vec<_> = variants.iter().map(|x| x.1).collect();

            make_read_and_write_logic_cstyle_enum(
                &variant_names,
                &variant_exprs,
                discriminant_type,
                &read_method,
                &write_method,
            )
        }
    };

    // Compute const field if proper arguments were given.
    let joined_const_field = match serializable_by_copy_creator {
        Some(serializable_by_copy_creator_fn) => {
            serializable_by_copy_creator_fn(&supported_kind, &trait_name)
        }
        None => quote! {},
    };

    // Collect to a beautiful implementation.
    quote! {
        #[automatically_derived]
        impl #impl_generics pbc_traits::#trait_name for #name #ty_generics #where_clause {
            #joined_const_field
            fn #read_method<#type_read: std::io::Read>(reader: &mut #type_read) -> Self {
                #read_logic
            }

            fn #write_method<#type_write: std::io::Write>(&self, writer: &mut #type_write) -> std::io::Result<()> {
                #write_logic
            }
        }
    }
}

/// Extracts the type from a field
///
/// * `field` - Field to extract type from.
pub fn field_to_type(field: &Field) -> TokenStream {
    let ty: TokenStream = match &field.ty {
        Type::Path(path) => path.clone().to_token_stream(),
        Type::Array(arr) => {
            let ident = match arr.elem.as_ref() {
                Type::Path(type_path) => type_path
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .clone()
                    .ident
                    .to_token_stream(),
                _ => unimplemented!("Not implemented"),
            };

            let len = match &arr.len {
                Expr::Lit(literal_expr) => Some(literal_expr.lit.to_token_stream()),
                _ => unimplemented!("Not implemented"),
            };

            parse(quote!([#ident; #len]))
        }
        _ => unimplemented!("Unknown type."),
    };
    ty.to_token_stream()
}

/// Attempts to convert the given AST element to [`SupportedKind`], an enum detailing the kind of
/// the annotated type.
///
/// May panic if the kind is unsupported.
fn get_kind_data(data: &Data) -> SupportedKind {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => SupportedKind::StructWithNamedFields(&fields.named),
            _ => unimplemented!("PBC serialization derives currently only supports named fields"),
        },
        Data::Enum(ref data) => {
            let mut variants = vec![];
            for x in &data.variants {
                match (&x.fields, &x.discriminant) {
                    (Fields::Unit, Some((_, expr))) => variants.push((&x.ident, expr)),
                    (_, _) => unimplemented!("PBC serialization derives only supports C-style enums with explicit discriminants"),
                }
            }
            SupportedKind::DiscriminatedCstyleEnum(format_ident!("u8"), variants)
        }
        _ => unimplemented!(
            "PBC serialization derives currently only support certain kinds of structs and enums"
        ),
    }
}

/// Describes the kind of the annotated type.
pub enum SupportedKind<'a> {
    /// Struct with named fields
    StructWithNamedFields(&'a syn::punctuated::Punctuated<syn::Field, syn::token::Comma>),
    /// C-style enum where all variants are annotated with explicit expressions.
    DiscriminatedCstyleEnum(syn::Ident, Vec<(&'a syn::Ident, &'a syn::Expr)>),
}

/// Implement read/write logic for a struct, specifically one with named fields.
///
/// This code is shared between `ReadWriteRPC` and `ReadWriteState`.
fn make_read_and_write_logic_struct(
    names: &[Ident],
    types: &[TokenStream],
    read_method: &Ident,
    write_method: &Ident,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    // For all (names, types) write `let name_n = type_n::read_method(reader)`.
    let read_lines = quote! {
        #(
            let #names =  <#types>::#read_method(reader);
        )*
        Self { #(#names),* }
    };

    // For all (names, types) write `self.field_n::write_method(reader)?`.
    let write_lines = quote! {
        #(
            <#types>::#write_method(&self.#names, writer)?;
        )*
        return Ok(());
    };

    (read_lines, write_lines)
}

/// Implement read/write logic for a C-style enum with discriminants.
///
/// This code is shared between `ReadWriteRPC` and `ReadWriteState`.
fn make_read_and_write_logic_cstyle_enum(
    variant_names: &[&syn::Ident],
    variant_expressions: &[&syn::Expr],
    discriminant_type: &Ident,
    read_method: &Ident,
    write_method: &Ident,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let read_lines = quote! {
        // Read, then match value.
        let __discriminant = #discriminant_type::#read_method(reader);
        let __matched_value = match __discriminant {
            #(
                #variant_expressions => Self::#variant_names,
            )*
            __unknown => panic!("No known enum value with discriminant {}", __unknown),
        };

        // This magic line ensures that discriminant type have been determined correctly at
        // compile-time, due to the type size check of transmute.
        // The assert should hopefully be optimized away, as it _should_ always be true at runtime.
        //
        // One unfortunate effect of this line is that the enum value must be PartialEq. Not a huge
        // problem, except for some mysterious error messages.
        assert!(__matched_value == unsafe { std::mem::transmute::<#discriminant_type,Self>(__discriminant) });
        __matched_value
    };

    // Generate match on each enum value, with write.
    let write_lines = quote! {
        let discriminant: #discriminant_type = match self {
            #(
                Self::#variant_names => #variant_expressions,
            )*
        };
        #discriminant_type::#write_method(&discriminant, writer)
    };

    (read_lines, write_lines)
}
