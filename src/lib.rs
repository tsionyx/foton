//! Simple photos and videos management tool.
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![forbid(unsafe_code)]

mod dir;
mod event;
mod file_types;
mod tags;

pub use self::{
    dir::Library,
    event::{
        time::{Format as TimeFormat, InfoSource as TimeSource, Time},
        Event,
    },
    file_types::MediaType,
    tags::{find_exif_tag, get_image_tags},
};

type AnyError = Box<dyn std::error::Error + Send + Sync>;
