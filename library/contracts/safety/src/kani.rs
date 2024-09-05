use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{Item, ItemFn, parse_macro_input};

pub(crate) fn requires(attr: TokenStream, item: TokenStream) -> TokenStream {
    rewrite_attr(attr, item, "requires")
}

pub(crate) fn ensures(attr: TokenStream, item: TokenStream) -> TokenStream {
    rewrite_attr(attr, item, "ensures")
}

pub(crate) fn invariant(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = proc_macro2::TokenStream::from(attr);
    let item = parse_macro_input!(item as Item);
    quote!(
        #[derive(kani_core::Arbitrary)]
        #[derive(kani_core::Invariant)]
        #[safety_constraint(#args)]
        #item
    ).into()
}

fn rewrite_attr(attr: TokenStream, item: TokenStream, name: &str) -> TokenStream {
    let args = proc_macro2::TokenStream::from(attr);
    let fn_item = parse_macro_input!(item as ItemFn);
    let attribute = format_ident!("{}", name);
    quote!(
        #[kani_core::#attribute(#args)]
        #fn_item
    ).into()
}
