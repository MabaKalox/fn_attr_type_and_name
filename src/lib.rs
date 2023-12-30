extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, TypeBareFn, BareVariadic, BareFnArg, FnArg};
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[proc_macro_attribute]
pub fn fn_type_and_name_attr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_string = quote!{ #fn_name }.to_string();
    let visibility = &input_fn.vis;

    let ident_fn_name = format_ident!("{}_NAME", fn_name);
    let ident_fn_type = format_ident!("{}_TYPE", fn_name);

    let bare_fn_type = item_fn_into_type_bare_fn(input_fn.clone());
    let type_func_type_stream = quote! {
        #[allow(non_camel_case_types)]
        #visibility type #ident_fn_type = #bare_fn_type;
    };
    let const_func_name_stream = quote! {
        #[allow(non_upper_case_globals)]
        #visibility const #ident_fn_name: &'static str = #fn_name_string;
    };

    let result = quote! {
        #input_fn
        #type_func_type_stream
        #const_func_name_stream
    };

    result.into()
}

fn item_fn_into_type_bare_fn(item_fn: ItemFn) -> TypeBareFn {
    let sig = item_fn.sig;
    let inputs = sig.inputs;
    let mut converted_inputs: Punctuated<BareFnArg, Comma> =  Punctuated::new();
    for input in inputs {
        let bare_fn_arg = match input {
            FnArg::Receiver(input) =>BareFnArg {
                attrs: input.attrs,
                name: None,
                ty: *input.ty
            },
            FnArg::Typed(input) => BareFnArg {
                attrs: input.attrs,
                name: None,
                ty: *input.ty,
            }
        };
        converted_inputs.push(bare_fn_arg);
    }
    TypeBareFn {
        lifetimes: None,
        unsafety: sig.unsafety,
        abi: sig.abi,
        fn_token: sig.fn_token,
        paren_token: sig.paren_token,
        inputs: converted_inputs,
        variadic: if let Some(v) = sig.variadic {
            Some(BareVariadic {
                attrs: v.attrs,
                name: None,
                dots: v.dots,
                comma: v.comma,
            })
        } else { None },
        output: sig.output,
    }
}