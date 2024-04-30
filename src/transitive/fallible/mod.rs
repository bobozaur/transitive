mod try_from;
mod try_into;

use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, Error as SynError, Expr, Meta, Path,
    Token,
};
pub use try_from::TryTransitionFrom;
pub use try_into::TryTransitionInto;

/// A path list that may contain a custom error type.
pub struct FalliblePathList {
    path_list: Vec<Path>,
    error: Option<Path>,
}

impl Parse for FalliblePathList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let meta_list = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;

        let mut path_list = Vec::with_capacity(meta_list.len());
        let mut error = None;

        let mut iter = meta_list.into_iter().peekable();

        while let Some(meta) = iter.next() {
            // If this is not the last element then it's definitely part of the path list
            if iter.peek().is_some() {
                match meta {
                    Meta::Path(path) => path_list.push(path),
                    _ => return Err(SynError::new(meta.span(), "invalid value in path list")),
                };
            } else {
                // We reached the last element which could be the custom error type
                match meta {
                    // Just a regular type path in the conversion path
                    Meta::Path(p) => path_list.push(p),
                    // Custom error, but must check that it's a type path
                    Meta::NameValue(n) if n.path.is_ident("error") => match n.value {
                        Expr::Path(p) => error = Some(p.path),
                        _ => return Err(SynError::new(n.value.span(), "error must be a type")),
                    },
                    _ => return Err(SynError::new(meta.span(), "invalid value in path list")),
                }
            }
        }

        let output = Self { path_list, error };
        Ok(output)
    }
}
