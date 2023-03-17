//! Internal Partisia Blockchain SDK for procedural macro utility.
//!
//! Common functions used in the procedural macros in the SDK.

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use std::collections::HashSet;
use std::hash::Hash;

use proc_macro2::{Literal, TokenStream};
use quote::ToTokens;
use syn::parse_quote::parse;
use syn::{Attribute, Data, Fields, FieldsNamed, Ident, Type};

/// Extracts a fields identifier
///
/// # Arguments
/// * `field` Field to extract identifier from.
pub fn field_to_name(field: &syn::Field) -> Ident {
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

/// Which methods to generate: read, write or both at the same time.
pub enum ReadWriteGenType {
    /// Only generate read functions
    Read,
    /// Only generate write functions
    Write,
    /// Generate both read and write functions
    Combined,
}

/// Implement a named trait that has read and a write methods.
/// The signature matches `ReadRPC`, `WriteRPC` and `ReadWriteState`.
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
    generation_type: ReadWriteGenType,
) -> TokenStream {
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
        SupportedKind::StructWithNamedFields { fields } => {
            let field_names: Vec<_> = fields
                .iter()
                .map(|x| field_to_name(x).to_token_stream())
                .collect();
            let write_field_names: Vec<_> =
                field_names.iter().map(|x| quote! { &self.#x }).collect();
            let field_types: Vec<_> = fields.iter().map(field_to_type).collect();

            make_read_and_write_logic_struct(
                quote! { Self },
                &field_names,
                &write_field_names,
                &field_types,
                &read_method,
                &write_method,
            )
        }
        SupportedKind::DiscriminatedCstyleEnum {
            discriminant_type,
            variants,
        } => {
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
        SupportedKind::ItemStructEnum {
            variant_discriminants,
            variant_names,
            variant_fields_types,
            variant_fields_names,
        } => {
            let mut read_variants: Vec<TokenStream> = vec![];
            let mut write_variants: Vec<TokenStream> = vec![];
            let mut variant_params: Vec<TokenStream> = vec![];
            for i in 0..variant_names.len() {
                let name: &syn::Ident = variant_names.get(i).unwrap();
                let field_names: &Vec<TokenStream> = variant_fields_names.get(i).unwrap();
                let field_types: &Vec<TokenStream> = variant_fields_types.get(i).unwrap();
                let read_write = make_read_and_write_logic_struct(
                    quote! { Self::#name },
                    field_names,
                    field_names,
                    field_types,
                    &read_method,
                    &write_method,
                );
                read_variants.push(read_write.0);
                write_variants.push(read_write.1);
                variant_params.push(quote! { #(#field_names),* });
            }

            make_read_and_write_item_struct_enum(
                variant_discriminants,
                variant_names,
                &variant_params,
                &read_variants,
                &write_variants,
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

    let read_block = quote! {
        #joined_const_field
        fn #read_method<#type_read: std::io::Read>(reader: &mut #type_read) -> Self {
            #read_logic
        }
    };

    let write_block = quote! {
        fn #write_method<#type_write: std::io::Write>(&self, writer: &mut #type_write) -> std::io::Result<()> {
            #write_logic
        }
    };

    match generation_type {
        ReadWriteGenType::Read => {
            // Collect to a beautiful implementation.
            quote! {
                #[automatically_derived]
                impl #impl_generics pbc_traits::#trait_name for #name #ty_generics #where_clause {
                    #read_block
                }
            }
        }

        ReadWriteGenType::Write => {
            // Collect to a beautiful implementation.
            quote! {
                #[automatically_derived]
                impl #impl_generics pbc_traits::#trait_name for #name #ty_generics #where_clause {
                    #write_block
                }
            }
        }

        ReadWriteGenType::Combined => {
            // Collect to a beautiful implementation.
            quote! {
                #[automatically_derived]
                impl #impl_generics pbc_traits::#trait_name for #name #ty_generics #where_clause {
                    #read_block
                    #write_block
                }
            }
        }
    }
}

/// Extracts the type from a field
///
/// * `field` - Field to extract type from.
pub fn field_to_type(field: &syn::Field) -> TokenStream {
    let ty: TokenStream = match &field.ty {
        Type::Path(path) => path.to_token_stream(),
        Type::Array(arr) => {
            let ident = match arr.elem.as_ref() {
                Type::Path(type_path) => type_path
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_token_stream(),
                _ => panic!("Unknown array element type"),
            };

            let len = match &arr.len {
                syn::Expr::Lit(literal_expr) => Some(literal_expr.lit.to_token_stream()),
                _ => panic!("The length of an array must be a literal"),
            };

            parse(quote!([#ident; #len]))
        }
        _ => panic!("Unknown type."),
    };
    ty.to_token_stream()
}

/// Attempts to convert the given AST element to [`SupportedKind`], an enum detailing the kind of
/// the annotated type.
///
/// May panic if the kind is unsupported.
pub fn get_kind_data(data: &Data) -> SupportedKind {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => SupportedKind::StructWithNamedFields { fields: &fields.named },
            _ => panic!("PBC serialization derives currently only supports named fields for structs"),
        },
        Data::Enum(ref data) => {
            let mut cstyle_variants = vec![];
            let mut struct_variants_discriminants: Vec<Literal> = vec![];
            let mut struct_variants_idents: Vec<&syn::Ident> = vec![];
            let mut struct_variants_field_types: Vec<Vec<TokenStream>> = vec![];
            let mut struct_variants_field_names: Vec<Vec<TokenStream>> = vec![];
            for x in &data.variants {
                match (&x.fields, &x.discriminant, &x.attrs, &x.ident) {
                    (Fields::Unit, Some((_, expr)), _, _) => cstyle_variants.push((&x.ident, expr)),
                    (Fields::Named(name), _, attrs, ident) => {
                        let (variant_discriminator, variant_fields_types, variant_fields_names) = extract_enum_variant_data(name, attrs);
                        struct_variants_discriminants.push(variant_discriminator);
                        struct_variants_idents.push(ident);
                        struct_variants_field_types.push(variant_fields_types);
                        struct_variants_field_names.push(variant_fields_names);
                    }
                    (_, _, _, _) => panic!("PBC serialization derives only supports explicitly discriminated C-style enums and enums consisting of struct variants"),
                }
            }
            if cstyle_variants.is_empty() {
                if has_unique_elements(struct_variants_discriminants.iter().map(|d| d.to_string())) {
                    SupportedKind::ItemStructEnum {
                        variant_discriminants: struct_variants_discriminants,
                        variant_names: struct_variants_idents,
                        variant_fields_types: struct_variants_field_types,
                        variant_fields_names: struct_variants_field_names,
                    }
                } else {
                    panic!("Duplicate discriminant values")
                }
            } else {
                SupportedKind::DiscriminatedCstyleEnum { discriminant_type: format_ident!("u8"), variants: cstyle_variants }
            }
        }
        _ => panic!("PBC serialization does not support Union, currently only certain kinds of structs and enums are supported")
    }
}

/// Check whether an iterator only has unique elements. If there exists a duplicate returns false.
pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

/// Extract variant data.
/// Returns a tuple consisting of the discriminator, variant_field_types, and variant_field_names.
///
/// May panic if the variant does not have a discriminant attribute.
pub fn extract_enum_variant_data(
    name: &FieldsNamed,
    attrs: &[Attribute],
) -> (Literal, Vec<TokenStream>, Vec<TokenStream>) {
    let discriminator_ident = format_ident!("discriminant");
    let variant_fields_types: Vec<TokenStream> = name.named.iter().map(field_to_type).collect();
    let variant_fields_names: Vec<&syn::Ident> = name
        .named
        .iter()
        .map(|x| {
            x.ident
                .as_ref()
                .expect("EnumItemStruct must have a name Identifier")
        })
        .collect();
    let variant_fields_names_ts: Vec<TokenStream> = variant_fields_names
        .iter()
        .map(|&x| x.to_token_stream())
        .collect();
    let variant_discriminator_stream: TokenStream = attrs
        .iter()
        .find(|attr| attr.path.is_ident(&discriminator_ident))
        .map(|attr| attr.tokens.to_token_stream())
        .expect("Attribute 'discriminant' is required for struct enum variants");
    let discriminator_with_paren: proc_macro2::Group = syn::parse2(variant_discriminator_stream)
        .unwrap_or_else(|_| {
            panic!("Discriminant is required to be a literal delimited by parenthesis")
        });
    let variant_discriminator: Literal = syn::parse2(discriminator_with_paren.stream())
        .unwrap_or_else(|_| panic!("Discriminant is required to be a literal"));
    (
        variant_discriminator,
        variant_fields_types,
        variant_fields_names_ts,
    )
}

/// Describes the kind of the annotated type.
pub enum SupportedKind<'a> {
    /// Struct with named fields
    StructWithNamedFields {
        /// The fields for the struct
        fields: &'a syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    },
    /// C-style enum where all variants are annotated with explicit expressions.
    DiscriminatedCstyleEnum {
        /// The discriminant type
        discriminant_type: syn::Ident,
        /// The list of variants
        variants: Vec<(&'a syn::Ident, &'a syn::Expr)>,
    },
    /// Enum where all variants are EnumItemStructs annotated with a discriminant attribute.
    ItemStructEnum {
        /// The discriminant of each variant
        variant_discriminants: Vec<Literal>,
        /// The name of each variant
        variant_names: Vec<&'a syn::Ident>,
        /// The field types for each variant
        variant_fields_types: Vec<Vec<TokenStream>>,
        /// The field names for each variant
        variant_fields_names: Vec<Vec<TokenStream>>,
    },
}

/// Implement read/write logic for a struct, specifically one with named fields.
///
/// This code is shared between `ReadRPC`, `WriteRPC` and `ReadWriteState`.
fn make_read_and_write_logic_struct(
    variant_constructor: TokenStream,
    names: &[TokenStream],
    write_names: &[TokenStream],
    types: &[TokenStream],
    read_method: &Ident,
    write_method: &Ident,
) -> (TokenStream, TokenStream) {
    // For all (names, types) write `let name_n = type_n::read_method(reader)`.
    let read_lines = quote! {
        #(
            let #names =  <#types>::#read_method(reader);
        )*
        #variant_constructor { #(#names),* }
    };

    // For all (names, types) write `self.field_n::write_method(reader)?`.
    let write_lines = quote! {
        #(
            <#types>::#write_method(#write_names, writer)?;
        )*
        Ok(())
    };

    (read_lines, write_lines)
}

/// Implement read/write logic for a C-style enum with discriminants.
///
/// This code is shared between `ReadRPC`, `WriteRPC` and `ReadWriteState`.
fn make_read_and_write_logic_cstyle_enum(
    variant_names: &[&syn::Ident],
    variant_expressions: &[&syn::Expr],
    discriminant_type: &Ident,
    read_method: &Ident,
    write_method: &Ident,
) -> (TokenStream, TokenStream) {
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

/// Implement read/write logic for a enum, specifically one with itemStruct variants.
///
/// This code is shared between `ReadRPC`, `WriteRPC` and `ReadWriteState`.
fn make_read_and_write_item_struct_enum(
    variant_discriminators: &[Literal],
    variant_names: &[&Ident],
    variant_params: &[TokenStream],
    read_variants: &[TokenStream],
    write_variants: &[TokenStream],
    read_method: &Ident,
    write_method: &Ident,
) -> (TokenStream, TokenStream) {
    let read_lines = quote! {
        let __discriminant = <u8>::#read_method(reader);
        let __matched_value = match __discriminant {
            #(
                #variant_discriminators => {
                    #read_variants
                },
            )*
            (__unknown) => panic!("No known enum value with discriminant {}", __unknown),
        };
        __matched_value
    };

    let write_lines = quote! {
        match self {
            #(Self::#variant_names { #variant_params } => {
                <u8>::#write_method(&#variant_discriminators, writer)?;
                #write_variants
            })*
        }
    };
    (read_lines, write_lines)
}
