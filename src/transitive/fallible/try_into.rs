#![allow(clippy::expect_fun_call)]

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

use crate::transitive::{arg_handler::ArgHandler, TRY_FROM};

pub struct TryIntoHandler;

impl ArgHandler for TryIntoHandler {
    fn conv_func_name(&self) -> &str {
        TRY_FROM
    }

    fn create_bulk_impl(
        &self,
        name: &Ident,
        stmts: TokenStream,
        first: Path,
        last: Path,
        second_last: Option<Path>,
    ) -> TokenStream {
        let second_last = second_last.as_ref().unwrap_or(&first);

        quote! {
            impl TryFrom<#name> for #last {
                type Error = <#last as TryFrom<#second_last>>::Error;

                fn try_from(val: #name) -> Result<Self, Self::Error> {
                    #stmts
                    Ok(interm)
                }
            }
        }
    }

    fn create_pair_impl(&self, name: &Ident, first: &Path, last: &Path) -> TokenStream {
        quote! {
            impl TryFrom<#name> for #last {
                type Error = <#last as TryFrom<#first>>::Error;

                fn try_from(val: #name) -> Result<Self, Self::Error> {
                    let interm = #first::try_from(val)?;
                    #last::try_from(interm)
                }
            }
        }
    }

    fn stmt_end(&self) -> TokenStream {
        quote! {?}
    }
}
