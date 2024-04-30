use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, punctuated::Punctuated, Path, Token};

use crate::transitive::TokenizableAttr;

pub struct TransitiveInto(Punctuated<Path, Token![,]>);

impl Parse for TransitiveInto {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Punctuated::parse_terminated(input).map(Self)
    }
}

impl ToTokens for TokenizableAttr<'_, &TransitiveInto> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let last = self.attr.0.last();

        let stmts = self
            .attr
            .0
            .iter()
            .take(self.attr.0.len() - 1)
            .map(|ty| quote! {let val: #ty = core::convert::From::from(val);});

        let expanded = quote! {
            impl #impl_generics core::convert::From<#name #ty_generics> for #last #where_clause {
                fn from(val: #name #ty_generics) -> #last {
                    #(#stmts)*
                    core::convert::From::from(val)
                }
            }
        };

        tokens.extend(expanded);
    }
}
