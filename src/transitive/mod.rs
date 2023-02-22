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

/// Attributes available for choosing a direction.
/// Should be fine to use an array as the lookup is really small.
/// If for some reason this grows, it would be a good idea to 
/// replace it with a lazy HashSet or something.
static VALID_ARGS: [&str; 4] = [FROM, INTO, TRY_FROM, TRY_INTO];
