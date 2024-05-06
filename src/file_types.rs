use std::{
    fmt,
    path::{Path, PathBuf},
};

use enum_iterator::Sequence;

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Sequence)]
/// High-level type of the photo-video resource.
pub enum MediaType {
    Photo,
    Animation,
    Video,
}

impl MediaType {
    /// Which file extensions associated with the type.
    pub fn supported_extensions(self) -> Vec<&'static str> {
        match self {
            Self::Photo => {
                vec!["jpg", "jpeg", "png"]
            }
            Self::Animation => {
                vec!["gif"]
            }
            Self::Video => {
                vec!["mp4"]
            }
        }
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self {
            Self::Photo => "PHOTO",
            Self::Animation => "ANIMATION",
            Self::Video => "VIDEO",
        };
        f.write_str(desc)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Represents a single media resource.
pub struct Media {
    pub(crate) type_: MediaType,
    pub(crate) path: PathBuf,
}

impl Media {
    /// The [type][MediaType] of the resource.
    pub fn type_(&self) -> MediaType {
        self.type_
    }

    /// Path to the media resource.
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

impl fmt::Display for Media {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]{}", self.type_, self.path.display())
    }
}
