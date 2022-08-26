use proc_macro2::TokenStream as TokenStream2;

#[derive(Debug)]
pub(crate) struct InstantiableArgument {
    /// Argument is treated as ignored when None.
    pub(crate) name: String,
    pub(crate) expression: TokenStream2,
}

impl InstantiableArgument {
    pub(crate) fn new(name: &str, expression: TokenStream2) -> InstantiableArgument {
        InstantiableArgument {
            name: name.to_owned(),
            expression,
        }
    }

    pub(crate) fn variable_name(&self) -> syn::Ident {
        format_ident!("__temporary_{}", self.name)
    }
}

pub(crate) struct ArgumentList {
    names: Vec<TokenStream2>,
    expressions: Vec<TokenStream2>,
}

impl ArgumentList {
    pub(crate) fn new() -> ArgumentList {
        ArgumentList {
            names: Vec::new(),
            expressions: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, name: TokenStream2, expression: TokenStream2) {
        self.names.push(name);
        self.expressions.push(expression);
    }

    pub(crate) fn convert_to_tuple(self) -> (Vec<String>, Vec<TokenStream2>) {
        let as_strings = self.names.iter().map(|token| token.to_string()).collect();

        (as_strings, self.expressions)
    }

    pub(crate) fn split_off(&mut self, at: usize) -> ArgumentList {
        ArgumentList {
            names: self.names.split_off(at),
            expressions: self.expressions.split_off(at),
        }
    }
}

pub(crate) struct TokenizedInvocation {
    pub(crate) context: InstantiableArgument,
    pub(crate) callback_context: Option<InstantiableArgument>,
    pub(crate) state: Option<InstantiableArgument>,
    pub(crate) zk_state: Option<InstantiableArgument>,
    pub(crate) rpc_params: Vec<InstantiableArgument>,

    /// Always tuple; empty list represents unit.
    pub(crate) result_types: Vec<syn::Type>,
}

impl TokenizedInvocation {
    pub(crate) fn new(
        context: InstantiableArgument,
        callback_context: Option<InstantiableArgument>,
        state: Option<InstantiableArgument>,
        zk_state: Option<InstantiableArgument>,
        rpc_params: Vec<InstantiableArgument>,
        result_types: Vec<syn::Type>,
    ) -> TokenizedInvocation {
        TokenizedInvocation {
            context,
            callback_context,
            state,
            zk_state,
            rpc_params,
            result_types,
        }
    }

    pub(crate) fn param_names(&self) -> Vec<syn::Ident> {
        self.zk_state
            .iter()
            .chain(self.rpc_params.iter())
            .map(|token| token.variable_name())
            .collect()
    }

    pub(crate) fn num_params(&self) -> usize {
        let num_well_known_params = vec![
            Some(&self.context),
            (&self.callback_context).as_ref(),
            (&self.state).as_ref(),
            (&self.zk_state).as_ref(),
        ]
        .iter()
        .filter(|x| x.is_some())
        .count();
        num_well_known_params + self.rpc_params.len()
    }

    pub(crate) fn param_instantiation_expr(&self) -> TokenStream2 {
        let rpc_param_names = self.param_names();

        let rpc_param_expressions: Vec<TokenStream2> = self
            .zk_state
            .iter()
            .chain(self.rpc_params.iter())
            .map(|token| token.expression.clone())
            .collect();

        let type_unknown = quote! { _ };
        let type_zk_state = quote! { pbc_contract_common::zk::ZkState<_> };
        let rpc_param_expected_types = {
            let mut types = vec![&type_unknown; rpc_param_expressions.len()];
            if self.zk_state.is_some() {
                types[0] = &type_zk_state;
            }
            types
        };

        // The expressions, which are used to evaluate the arguments for the inner function,
        // deserialize from "cursor" meaning they have side effects.
        // Because of this, we need to ensure that they are evaluated in the correct order,
        // thus we will bind them to variables instead of #fn_identifier(#(#expression),*)
        // (since function arguments are not guaranteed to evaluate left to right).
        quote! {
            #(let #rpc_param_names: #rpc_param_expected_types = #rpc_param_expressions)*
        }
    }
}
