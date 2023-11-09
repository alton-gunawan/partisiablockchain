use proc_macro::TokenStream;
use quote::ToTokens;

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

    let read_block = quote! {
        #(
            let #field_names = <#field_types>::secret_read_from(reader);
        )*
        Self { #(#field_names),* }
    };

    let write_block = quote! {
        #(
            <#field_types>::secret_write_to(&self.#field_names, writer)?;
        )*
        Ok(())
    };

    let impl_secret_block = quote! {
        #[automatically_derived]
        impl ::pbc_zk::SecretBinary for #type_name {
            fn secret_read_from<T: std::io::Read>(reader: &mut T) -> Self {
                #read_block
            }
            fn secret_write_to<T: std::io::Write>(&self, writer: &mut T) -> std::io::Result<()> {
                #write_block
            }
        }
        #[automatically_derived]
        impl #type_name {
            #[automatically_derived]
            #[allow(dead_code)]
            fn __ignore_for_secret_binary() {
                fn ignore<T: ::pbc_zk::SecretBinary>() {}
                #(
                    ignore::<#field_types>();
                )*
            }
        };
    };

    // Return the generated impl
    impl_secret_block.into()
}
