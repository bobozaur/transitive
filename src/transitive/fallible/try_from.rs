#![allow(clippy::expect_fun_call)]

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

use crate::transitive::{arg_handler::ArgHandler, TRY_INTO};

pub struct TryFromHandler;

impl ArgHandler for TryFromHandler {
    fn conv_func_name(&self) -> &str {
        TRY_INTO
    }

    fn create_bulk_impl(
        &self,
        name: &Ident,
        stmts: TokenStream,
        first: Path,
        last: Path,
        _second_last: Option<Path>,
    ) -> TokenStream {
        quote! {
            impl TryFrom<#first> for #name {
                type Error = <#name as TryFrom<#last>>::Error;

                fn try_from(val: #first) -> Result<Self, Self::Error> {
                    #stmts
                    Ok(interm)
                }
            }
        }
    }

    fn create_pair_impl(&self, name: &Ident, first: &Path, last: &Path) -> TokenStream {
        quote! {
            impl TryFrom<#first> for #name {
                type Error = <#name as TryFrom<#last>>::Error;

                fn try_from(val: #first) -> Result<Self, Self::Error> {
                    let interm = #last::try_from(val)?;
                    #name::try_from(interm)
                }
            }
        }
    }

    fn stmt_end(&self) -> TokenStream {
        quote! {?}
    }
}
