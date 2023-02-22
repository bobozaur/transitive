mod arg_handler;
mod arg_list_type;
mod direction;
pub mod direction_handler;
pub mod fallible;
pub mod infallible;
mod minimal_args;

use std::iter::Map;

use syn::{
    punctuated::{IntoIter, Punctuated},
    NestedMeta, Path, Result as SynResult, Token,
};

type RawArgList = Punctuated<NestedMeta, Token![,]>;
type ArgsIter = Map<IntoIter<NestedMeta>, ArgMapFn>;
type ArgMapFn = fn(NestedMeta) -> SynResult<Path>;

const TRANSITIVE: &str = "transitive";
const ALL: &str = "all";
const FROM: &str = "from";
const INTO: &str = "into";
const TRY_INTO: &str = "try_into";
const TRY_FROM: &str = "try_from";
