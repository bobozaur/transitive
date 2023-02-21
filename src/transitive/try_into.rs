#![allow(clippy::expect_fun_call)]

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;


pub fn ts_maker(stmts: TokenStream, name: &Ident, first: Path, last: Path) -> TokenStream {
    quote! {
        impl TryFrom<#name> for #last {
            type Error = <#last as TryFrom<#first>>::Error;

            fn try_from(val: #name) -> Result<Self, Self::Error> {
                #stmts
                Ok(interm)
            }
        }
    }
}

pub fn create_try_from_impl(name: &Ident, interm: &Path, target: &Path) -> TokenStream {
    quote! {
        impl TryFrom<#name> for #target {
            type Error = <#target as TryFrom<#interm>>::Error;

            fn try_from(val: #name) -> Result<Self, Self::Error> {
                let interm = #interm::try_from(val)?;
                #target::try_from(interm)
            }
        }
    }
}
