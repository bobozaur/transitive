#![allow(clippy::expect_fun_call)]

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

pub fn ts_maker(stmts: TokenStream, name: &Ident, first: Path, last: Path, _second_last: Option<Path>) -> TokenStream {
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

pub fn create_try_from_impl(name: &Ident, first: &Path, last: &Path) -> TokenStream {
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
