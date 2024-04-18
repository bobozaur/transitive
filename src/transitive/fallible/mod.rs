mod try_from;
mod try_into;

use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, Error as SynError, Expr, Meta, Path,
    Token,
};
pub use try_from::TransitiveTryFrom;
pub use try_into::TransitiveTryInto;

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
                let Meta::Path(path) = meta else {
                    return Err(SynError::new(meta.span(), "invalid value in the path list"));
                };

                path_list.push(path);
            } else {
                // We reached the last element which could be the custom error type
                match meta {
                    Meta::Path(p) => path_list.push(p),
                    Meta::NameValue(n) => match n.path.get_ident() {
                        Some(i) if i == "error" => match n.value {
                            Expr::Path(p) => error = Some(p.path),
                            _ => {
                                return Err(SynError::new(
                                    n.value.span(),
                                    "invalid value in the path list",
                                ))
                            }
                        },
                        _ => {
                            return Err(SynError::new(
                                n.path.span(),
                                "invalid value in the path list",
                            ))
                        }
                    },
                    _ => return Err(SynError::new(meta.span(), "invalid value in the path list")),
                }
            }
        }

        let output = Self { path_list, error };
        Ok(output)
    }
}
