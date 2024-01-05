use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_quote;

pub(crate) fn implement_secret(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let type_name = &ast.ident;
    let supported_kind = derive_commons::get_kind_data(&ast.data);

    let (field_names, field_types): (Vec<_>, Vec<_>) = match supported_kind {
        derive_commons::SupportedKind::StructWithNamedFields { fields, .. } => {
            let field_names: Vec<_> = fields
                .iter()
                .map(|x| derive_commons::field_to_name(x).to_token_stream())
                .collect();
            let field_types: Vec<_> = fields.iter().map(derive_commons::field_to_type).collect();
            (field_names, field_types)
        }
        derive_commons::SupportedKind::DiscriminatedCstyleEnum { .. } => (vec![], vec![]),
        derive_commons::SupportedKind::ItemStructEnum { .. } => unimplemented!("ItemStructEnum"),
    };

    let impl_fixed_size = {
        let trait_name: syn::Path = parse_quote! { ::pbc_zk::SecretBinaryFixedSize };

        let mut generics = ast.generics.clone();
        derive_commons::extend_generic_bounds_with_trait(&mut generics, &trait_name);
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let bits = quote! { #( <#field_types as #trait_name>::BITS +)* 0 };

        quote! {
            #[automatically_derived]
            impl #impl_generics #trait_name for #type_name #ty_generics #where_clause {
                const BITS: u32 = #bits;
            }
        }
    };

    let impl_secret_block = {
        let trait_name: syn::Path = parse_quote! { ::pbc_zk::SecretBinary };
        let mut generics = ast.generics.clone();
        derive_commons::extend_generic_bounds_with_trait(&mut generics, &trait_name);
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let read_block = quote! {
            #(
                let #field_names = <#field_types as #trait_name>::secret_read_from(reader);
            )*
            Self { #(#field_names),* }
        };

        let write_block = quote! {
            #(
                <#field_types as #trait_name>::secret_write_to(&self.#field_names, writer)?;
            )*
            Ok(())
        };

        quote! {
            #[automatically_derived]
            impl #impl_generics #trait_name for #type_name #ty_generics #where_clause {
                fn secret_read_from<ReadT: std::io::Read>(reader: &mut ReadT) -> Self {
                    #read_block
                }
                fn secret_write_to<WriteT: std::io::Write>(&self, writer: &mut WriteT) -> std::io::Result<()> {
                    #write_block
                }
            }

            #[automatically_derived]
            impl #impl_generics #type_name #ty_generics #where_clause {
                #[automatically_derived]
                #[allow(dead_code)]
                fn __ignore_for_secret_binary() {
                    fn ignore<IgnoreT: #trait_name>() {}
                    #(
                        ignore::<#field_types>();
                    )*
                }
            };
        }
    };

    // Return the generated impl
    quote! {
        #impl_fixed_size

        #impl_secret_block
    }
    .into()
}
