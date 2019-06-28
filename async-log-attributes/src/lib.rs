//! Proc Macro attributes for the `async-log` crate.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// Defines the async main function.
///
/// # Examples
///
/// ```ignore
/// #[span]
/// async fn main() -> std::io::Result<()> {
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn instrument(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let constness = &input.constness;
    let unsafety = &input.unsafety;
    let asyncness = &input.asyncness;
    let abi = &input.abi;

    let generics = &input.decl.generics;
    let name = &input.ident;
    let inputs = &input.decl.inputs;
    let output = &input.decl.output;
    let body = &input.block.stmts;

    let args: Vec<syn::Pat> = inputs
        .pairs()
        .filter_map(|pair| match pair.into_value() {
            syn::FnArg::Captured(arg) => Some(arg.pat.clone()),
            _ => return None,
        })
        .collect();

    let names: String = args
        .iter()
        .enumerate()
        .map(|(i, _arg)| {
            let mut string = format!(", arg_{}", i);
            string.push_str("={}");
            string
        })
        .collect();

    let result = quote! {
        #(#attrs)*
        #vis #constness #unsafety #asyncness #abi fn #generics #name(#(#inputs)*) #output {
            let __name = format!("{}#{}", file!(), stringify!(#name));
            let __args = format!("{}{}", __name, format_args!(#names, #(#args)*));
            async_log::span!(__args, {
                #(#body)*
            })
        }
    };

    result.into()
}
