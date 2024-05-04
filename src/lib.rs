//! Simple photos and videos management tool.
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![forbid(unsafe_code)]

mod dir;
mod file_types;
mod tags;

pub use self::{
    dir::Library,
    file_types::MediaType,
    tags::{get_image_tags, get_tag_values_distribution, get_tags_distribution, get_video_tags},
};

type AnyError = Box<dyn std::error::Error + Send + Sync>;
