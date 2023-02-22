use self::{from::FromHandler, into::IntoHandler};

use super::{
    direction_handler::{DirectionHandler, DirectionKind},
    FROM, INTO,
};

mod from;
mod into;

pub struct InfallibleTransition;

impl DirectionHandler for InfallibleTransition {
    type IntoHandler = IntoHandler;

    type FromHandler = FromHandler;

    type Kind = InfallibleDirection;

    fn handler_into(&self) -> Self::IntoHandler {
        IntoHandler
    }

    fn handler_from(&self) -> Self::FromHandler {
        FromHandler
    }
}

pub struct InfallibleDirection;

impl DirectionKind for InfallibleDirection {
    fn arg_from() -> &'static str {
        FROM
    }

    fn arg_into() -> &'static str {
        INTO
    }
}
