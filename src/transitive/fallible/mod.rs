use self::{try_from::TryFromHandler, try_into::TryIntoHandler};

use super::{
    direction_handler::{DirectionHandler, DirectionKind},
    TRY_FROM, TRY_INTO,
};

mod try_from;
mod try_into;

pub struct FallibleTransition;

impl DirectionHandler for FallibleTransition {
    type IntoHandler = TryIntoHandler;

    type FromHandler = TryFromHandler;

    type Kind = FallibleDirection;

    fn handler_into(&self) -> Self::IntoHandler {
        TryIntoHandler
    }

    fn handler_from(&self) -> Self::FromHandler {
        TryFromHandler
    }
}

pub struct FallibleDirection;

impl DirectionKind for FallibleDirection {
    fn arg_from() -> &'static str {
        TRY_FROM
    }

    fn arg_into() -> &'static str {
        TRY_INTO
    }
}
