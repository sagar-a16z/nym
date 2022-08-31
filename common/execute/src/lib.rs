use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Block, ExprMethodCall, FnArg, Ident, ItemFn, LitStr, ReturnType, Token,
    VisPublic, Visibility,
};

#[proc_macro_attribute]
pub fn execute(attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as ItemFn);
    let target = parse_macro_input!(attr as LitStr).value();

    let cl = if target == "mixnet" {
        quote! {self.mixnet_contract_address()}
    } else if target == "vesting" {
        quote! {self.vesting_contract_address()}
    } else {
        panic!("Only `mixnet` and `vesting` targets are supported!")
    };
    let cl = proc_macro::TokenStream::from(cl);
    let cl = parse_macro_input!(cl as ExprMethodCall);

    let orig_f = f.clone();
    let mut execute_f = f.clone();
    let mut simulate_f = f.clone();
    let name = f.sig.ident;
    let name_str = name.to_string();
    let call_args = f.sig.inputs.into_iter().filter_map(|arg| match arg {
        FnArg::Receiver(_) => None,
        FnArg::Typed(arg) => Some(arg.pat),
    });
    let execute_args = call_args.clone();
    let simulate_args = call_args;

    execute_f.sig.asyncness = Some(Token![async](execute_f.sig.ident.span()));
    simulate_f.sig.asyncness = Some(Token![async](simulate_f.sig.ident.span()));

    execute_f.vis = Visibility::Public(VisPublic {
        pub_token: Token![pub](execute_f.sig.ident.span()),
    });
    simulate_f.vis = Visibility::Public(VisPublic {
        pub_token: Token![pub](simulate_f.sig.ident.span()),
    });

    execute_f.sig.ident = Ident::new(
        &format!("execute{}", execute_f.sig.ident),
        execute_f.sig.ident.span(),
    );

    simulate_f.sig.ident = Ident::new(
        &format!("simulate{}", simulate_f.sig.ident),
        simulate_f.sig.ident.span(),
    );

    let execute_output = quote! {
        -> Result<ExecuteResult, NymdError>
    };
    let o_ts = proc_macro::TokenStream::from(execute_output);
    execute_f.sig.output = parse_macro_input!(o_ts as ReturnType);

    let simulate_output = quote! {
        -> Result<SimulateResponse, NymdError>
    };
    let o_ts = proc_macro::TokenStream::from(simulate_output);
    simulate_f.sig.output = parse_macro_input!(o_ts as ReturnType);

    let simulate_block = quote! {
        {
            let (msg, _fee) = self.#name(#(#simulate_args),*);
            let msg = self.wrap_contract_execute_message(
                #cl,
                &msg,
                vec![],
            )?;

            self.simulate(vec![msg]).await
        }
    };

    let ts = proc_macro::TokenStream::from(simulate_block);
    simulate_f.block = Box::new(parse_macro_input!(ts as Block));

    let execute_block = quote! {
        {
            let (req, fee) = self.#name(#(#execute_args),*);
            let fee = fee.unwrap_or(Fee::Auto(Some(self.simulated_gas_multiplier)));
            self.client
                .execute(
                    self.address(),
                    #cl,
                    &req,
                    fee,
                    #name_str,
                    vec![],
                )
                .await
        }
    };

    let ts = proc_macro::TokenStream::from(execute_block);
    execute_f.block = Box::new(parse_macro_input!(ts as Block));

    let out = quote! {
        #orig_f
        #execute_f
        #simulate_f
    };

    out.into()
}

#[proc_macro_attribute]
pub fn offline(attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as ItemFn);
    let target = parse_macro_input!(attr as LitStr).value();

    let cl = if target == "mixnet" {
        quote! {self.mixnet_contract_address()}
    } else if target == "vesting" {
        quote! {self.vesting_contract_address()}
    } else {
        panic!("Only `mixnet` and `vesting` targets are supported!")
    };
    let cl = proc_macro::TokenStream::from(cl);
    let cl = parse_macro_input!(cl as ExprMethodCall);

    // need to export the original as well otherwise the `_some_func` name won't be known
    let orig_f = f.clone();
    let mut execute_f = f.clone();
    let mut simulate_f = f.clone();
    let name = f.sig.ident;
    let name_str = name.to_string();
    let call_args = f.sig.inputs.into_iter().filter_map(|arg| match arg {
        FnArg::Receiver(_) => None,
        FnArg::Typed(arg) => Some(arg.pat),
    });
    let execute_args = call_args.clone();
    let simulate_args = call_args;

    execute_f.sig.asyncness = None;
    simulate_f.sig.asyncness = Some(Token![async](simulate_f.sig.ident.span()));

    execute_f.vis = Visibility::Public(VisPublic {
        pub_token: Token![pub](execute_f.sig.ident.span()),
    });
    simulate_f.vis = Visibility::Public(VisPublic {
        pub_token: Token![pub](simulate_f.sig.ident.span()),
    });

    execute_f.sig.ident = Ident::new(
        &format!("execute{}", execute_f.sig.ident),
        execute_f.sig.ident.span(),
    );

    simulate_f.sig.ident = Ident::new(
        &format!("simulate{}", simulate_f.sig.ident),
        simulate_f.sig.ident.span(),
    );

    let execute_output = quote! {
        -> Result<ExecuteResult, NymdError>
    };
    let o_ts = proc_macro::TokenStream::from(execute_output);
    execute_f.sig.output = parse_macro_input!(o_ts as ReturnType);

    let simulate_output = quote! {
        -> Result<SimulateResponse, NymdError>
    };
    let o_ts = proc_macro::TokenStream::from(simulate_output);
    simulate_f.sig.output = parse_macro_input!(o_ts as ReturnType);

    let simulate_block = quote! {
        {
            let (msg, args) = self.#name(#(#simulate_args),*);
            let msg = self.wrap_contract_execute_message(
                #cl,
                &msg,
                args.funds.into_iter().map(Into::into).collect(),
            )?;

            self.simulate(vec![msg]).await
        }
    };

    let ts = proc_macro::TokenStream::from(simulate_block);
    simulate_f.block = Box::new(parse_macro_input!(ts as Block));

    let execute_block = quote! {
        {
            let (req, args) = self.#name(#(#execute_args),*);
            let denom = self.current_config().chain_details.mix_denom.base.clone();
            let fee = match args.fee {
                Some(fee) => fee,
                None => {
                    // TODO what's this supposed to be for offline signing? Pretty sure this isn't right.
                    let fee_amount = Coin {
                        amount: 25_000,
                        denom,
                    };
                    // Can't use automatic fee because that needs network access
                    tx::Fee::from_amount_and_gas(fee_amount.into(), 100_0000)
                }
            };
            let execute_msg = cosmwasm::MsgExecuteContract {
                sender: self.address().clone(),
                contract: #cl.clone(),
                msg: serde_json::to_vec(&req)?,
                funds: args.funds.into_iter().map(Into::into).collect(),
            }
            .to_any()
            .map_err(|_| NymdError::SerializationError("MsgExecuteContract".to_owned()))?;
            let result = self.client
                .offline_sign(
                    self.address(),
                    vec![execute_msg],
                    fee,
                    #name_str,
                    args.account_number,
                    args.sequence_number,
                    args.chain_id,
                )?;
            let tx_bytes = result.to_bytes().unwrap();
            Ok(ExecuteResult {
                logs: vec![],
                data: Data::from(tx_bytes),
                transaction_hash: tx::Hash::new([0u8; 32]),
                gas_info: Default::default(),
            })
        }
    };

    let ts = proc_macro::TokenStream::from(execute_block);
    execute_f.block = Box::new(parse_macro_input!(ts as Block));

    let out = quote! {
        #orig_f
        #execute_f
        #simulate_f
    };

    out.into()
}
