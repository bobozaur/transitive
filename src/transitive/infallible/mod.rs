use self::{from::FromHandler, into::IntoHandler};

use super::direction_handler::DirectionHandler;

mod from;
mod into;

pub struct InfallibleTransition;

impl DirectionHandler for InfallibleTransition {
    type IntoHandler = IntoHandler;

    type FromHandler = FromHandler;

    fn make_into_handler(&self) -> Self::IntoHandler {
        IntoHandler
    }

    fn make_from_handler(&self) -> Self::FromHandler {
        FromHandler
    }
}
