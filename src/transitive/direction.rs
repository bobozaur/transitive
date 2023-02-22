use std::marker::PhantomData;

use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Meta, MetaList, NestedMeta, Result as SynResult,
};

use super::{arg_list_type::ArgListType, direction_handler::DirectionKind};

pub enum Direction {
    From(ArgListType),
    Into(ArgListType),
}

pub struct DirectionWrapper<K>
where
    K: DirectionKind,
{
    direction: Direction,
    marker: PhantomData<fn() -> K>,
}

impl<K> DirectionWrapper<K>
where
    K: DirectionKind,
{
    pub fn into_inner(self) -> Direction {
        self.direction
    }

    fn direction_from(args: ArgListType) -> Self {
        Self {
            direction: Direction::From(args),
            marker: PhantomData,
        }
    }

    fn direction_into(args: ArgListType) -> Self {
        Self {
            direction: Direction::Into(args),
            marker: PhantomData,
        }
    }
}

impl<K> TryFrom<NestedMeta> for DirectionWrapper<K>
where
    K: DirectionKind,
{
    type Error = Error;

    fn try_from(value: NestedMeta) -> Result<Self, Self::Error> {
        let NestedMeta::Meta(Meta::List(list)) = value else {
            return Err(Error::new(value.span(), "must provide a direction"))
        };

        list.try_into()
    }
}

impl<K> TryFrom<MetaList> for DirectionWrapper<K>
where
    K: DirectionKind,
{
    type Error = Error;

    fn try_from(value: MetaList) -> Result<Self, Self::Error> {
        match value.path.get_ident() {
            Some(i) if i == K::arg_from() => Ok(Self::direction_from(value.nested.try_into()?)),
            Some(i) if i == K::arg_into() => Ok(Self::direction_into(value.nested.try_into()?)),
            Some(i) => Err(Error::new(i.span(), format!("unknown argument {i}"))),
            None => Err(Error::new(value.path.span(), "missing direction argument")),
        }
    }
}

/// Implementing [`Parse`] to directly parse the given attribute.
/// The attribute must always have arguments.
///
/// The primary arguments decide the direction: `from()` or `into()`.
/// These contain a types list, optionally encapsulated in
/// `all()`
///
/// The type lists must have at least two arguments, otherwise
/// there's no transition to do.
impl<K> Parse for DirectionWrapper<K>
where
    K: DirectionKind,
{
    fn parse(input: ParseStream) -> SynResult<Self> {
        let nested = NestedMeta::parse(input)?;
        nested.try_into()
    }
}
