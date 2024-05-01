use clap::ValueEnum;
use enum_iterator::Sequence;

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Sequence, ValueEnum)]
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
