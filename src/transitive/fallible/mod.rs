use self::{try_from::TryFromHandler, try_into::TryIntoHandler};

use super::direction_handler::DirectionHandler;

mod try_from;
mod try_into;

pub struct FallibleTransition;

impl DirectionHandler for FallibleTransition {
    type IntoHandler = TryIntoHandler;

    type FromHandler = TryFromHandler;

    fn make_into_handler(&self) -> Self::IntoHandler {
        TryIntoHandler
    }

    fn make_from_handler(&self) -> Self::FromHandler {
        TryFromHandler
    }
}
