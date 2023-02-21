#![allow(clippy::expect_fun_call)]

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

pub fn ts_maker(stmts: TokenStream, name: &Ident, _first: Path, last: Path) -> TokenStream {
    quote! {
        impl From<#name> for #last {
            fn from(val: #name) -> #last {
                #stmts
                interm
            }
        }
    }
}

pub fn create_from_impl(name: &Ident, interm: &Path, target: &Path) -> TokenStream {
    quote! {
        impl From<#name> for #target {
            fn from(val: #name) -> #target {
                let interm = #interm::from(val);
                #target::from(interm)
            }
        }
    }
}
