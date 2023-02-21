#![allow(clippy::expect_fun_call)]

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

pub fn ts_maker(stmts: TokenStream, name: &Ident, first: Path, _last: Path, _second_last: Option<Path>) -> TokenStream {
    quote! {
        impl From<#first> for #name {
            fn from(val: #first) -> #name {
                #stmts
                interm
            }
        }
    }
}

pub fn create_from_impl(name: &Ident, first: &Path, last: &Path) -> TokenStream {
    quote! {
        impl From<#first> for #name {
            fn from(val: #first) -> #name {
                let interm = #last::from(val);
                #name::from(interm)
            }
        }
    }
}
