//! The cfg fake main macro.

#![feature(proc_macro_expand)]

extern crate proc_macro;
use proc_macro::TokenStream;

/// The macro that does what `cfg` does but also injects a fake `fn main` when
/// the predicate is `false`.
#[proc_macro_attribute]
pub fn cfg_(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let item = proc_macro2::TokenStream::from(item);

    let attr_text = attr.to_string();

    let to_expand: proc_macro2::TokenStream = quote::quote! {
        ::core::cfg!( #attr )
    };
    let to_expand: TokenStream = to_expand.into();
    let expanded = to_expand.expand_expr().unwrap();

    let enabled = expanded.to_string() == "true";

    if enabled {
        quote::quote! {
            #![feature(prelude_import)]
            #![allow(internal_features)]
            #![allow(unused_imports)]

            #item
        }
        .into()
    } else {
        quote::quote! {
            #![feature(prelude_import)]
            #![allow(internal_features)]

            #[macro_use]
            extern crate std;

            #[prelude_import]
            use std::prelude::v1::*;

            fn main() {
                unimplemented!(#attr_text);
            }
        }
        .into()
    }
}
