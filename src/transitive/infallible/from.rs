#![allow(clippy::expect_fun_call)]

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

use crate::transitive::{arg_handler::ArgHandler, INTO};

pub struct FromHandler;

impl ArgHandler for FromHandler {
    fn conv_func_name(&self) -> &str {
        INTO
    }

    fn create_bulk_impl(
        &self,
        name: &Ident,
        stmts: TokenStream,
        first: Path,
        _last: Path,
        _second_last: Option<Path>,
    ) -> TokenStream {
        quote! {
            impl core::convert::From<#first> for #name {
                fn from(val: #first) -> #name {
                    #stmts
                    interm
                }
            }
        }
    }

    fn create_pair_impl(&self, name: &Ident, first: &Path, last: &Path) -> TokenStream {
        quote! {
            impl core::convert::From<#first> for #name {
                fn from(val: #first) -> #name {
                    let interm = #last::from(val);
                    #name::from(interm)
                }
            }
        }
    }
}
