//! Space and time coordinates (called also _event_)
//! for media files.

mod space;
pub(crate) mod time;

/// [`Event`] is a combination
/// of space (location) and time coordinates.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Event {
    time: Option<time::Time>,
    location: Option<space::Location>,
}
