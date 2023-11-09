use proc_macro::TokenStream;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::token::{Comma, Semi};
use syn::{bracketed, Expr, LitInt};

/// Information of a secret test
struct SecretTest {
    /// The expression to be tested.
    test: Expr,
    /// The expected result.
    result: Expr,
    /// List of secret inputs.
    secret_inputs: Vec<Expr>,
    /// List of expected secret outputs.
    secret_outputs: Vec<Expr>,
}

impl Parse for SecretTest {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let test: Expr = Expr::parse(input).unwrap();
        Comma::parse(input).unwrap();
        let result = Expr::parse(input).unwrap();
        let mut comma = Comma::parse(input);
        if comma.is_err() {
            return Ok(SecretTest {
                test,
                result,
                secret_inputs: vec![],
                secret_outputs: vec![],
            });
        }
        let content;
        bracketed!(content in input);
        let secret_inputs = parse_array(content);
        comma = Comma::parse(input);
        if comma.is_err() {
            return Ok(SecretTest {
                test,
                result,
                secret_inputs,
                secret_outputs: vec![],
            });
        }
        let content2;
        bracketed!(content2 in input);
        let secret_outputs = parse_array(content2);
        Ok(SecretTest {
            test,
            result,
            secret_inputs,
            secret_outputs,
        })
    }
}

fn parse_array(content: ParseBuffer) -> Vec<Expr> {
    let content_fork = content.fork();
    let secret_inputs_list =
        syn::punctuated::Punctuated::<Expr, Comma>::parse_terminated(&content_fork);
    if let Ok(secret_inputs_list) = secret_inputs_list {
        content.advance_to(&content_fork);
        secret_inputs_list.into_iter().collect()
    } else {
        let value: Expr = Expr::parse(&content).unwrap();
        Semi::parse(&content).unwrap();
        let size = LitInt::parse(&content).unwrap();
        vec![value; size.base10_parse::<usize>().unwrap()]
    }
}

pub(crate) fn implement_test_eq(input: TokenStream) -> TokenStream {
    let ast: SecretTest = syn::parse(input.clone()).unwrap();
    let test = ast.test;
    let result = ast.result;
    let secret_inputs = ast.secret_inputs.as_slice();
    let secret_outputs = ast.secret_outputs.as_slice();
    let indices: Vec<usize> = (0..secret_outputs.len()).collect();
    let indent_str: String = input
        .to_string()
        .replace(|c: char| c.is_whitespace(), "")
        .replace(|c: char| !c.is_alphanumeric(), "_");
    let ident = format_ident!("test_{}", indent_str);
    let out = quote! {
        #[test]
        fn #ident() {
           let secret_inputs = vec![
                #(
                pbc_zk::api::SecretVar {
                    value: (#secret_inputs).to_le_bytes().to_vec(),
                    metadata: (#secret_inputs).to_le_bytes().to_vec(),
                }),*
                ];

            unsafe {
                pbc_zk::api::set_secrets(secret_inputs);
            }

            assert_eq!(#test, (#result).into());
            let secret_outputs = pbc_zk::api::get_secret_outputs();
            #(
                assert!((#secret_outputs).to_le_bytes().starts_with(&secret_outputs.get(#indices).unwrap().value));
            )*
        }
    };
    out.into()
}
