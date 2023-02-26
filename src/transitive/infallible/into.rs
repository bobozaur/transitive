#![allow(clippy::expect_fun_call)]
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

use crate::transitive::{arg_handler::ArgHandler, FROM};

pub struct IntoHandler;

impl ArgHandler for IntoHandler {
    fn conv_func_name(&self) -> &str {
        FROM
    }

    fn create_bulk_impl(
        &self,
        name: &Ident,
        stmts: TokenStream,
        _first: Path,
        last: Path,
        _second_last: Option<Path>,
    ) -> TokenStream {
        quote! {
            impl core::convert::From<#name> for #last {
                fn from(val: #name) -> #last {
                    #stmts
                    interm
                }
            }
        }
    }

    fn create_pair_impl(&self, name: &Ident, first: &Path, last: &Path) -> TokenStream {
        quote! {
            impl core::convert::From<#name> for #last {
                fn from(val: #name) -> #last {
                    let interm = #first::from(val);
                    #last::from(interm)
                }
            }
        }
    }
}
