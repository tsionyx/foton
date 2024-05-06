use clap::{builder::PossibleValue, Args, Parser, Subcommand, ValueEnum};

use foton::MediaType;

#[derive(Debug, Clone, Parser)]
/// Manage the photos and videos collection.
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub(crate) enum Command {
    /// List files in a collection.
    List {
        /// Type of the resource to find.
        type_: Option<PrivateMediaType>,
    },

    /// View or create a configuration file.
    Config(ConfigArgs),

    /// Show metadata.
    Tags(TagArgs),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum PrivateMediaType {
    Photo,
    Animation,
    Video,
}

impl From<PrivateMediaType> for MediaType {
    fn from(value: PrivateMediaType) -> Self {
        match value {
            PrivateMediaType::Photo => Self::Photo,
            PrivateMediaType::Animation => Self::Animation,
            PrivateMediaType::Video => Self::Video,
        }
    }
}

impl ValueEnum for PrivateMediaType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Photo, Self::Animation, Self::Video]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Photo => PossibleValue::new("photo"),
            Self::Animation => PossibleValue::new("animation"),
            Self::Video => PossibleValue::new("video"),
        })
    }
}

#[derive(Debug, Copy, Clone, Args)]
pub(crate) struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Debug, Copy, Clone, Subcommand)]
pub(crate) enum ConfigCommand {
    /// Print the current configuration file in use (if any).
    Print,

    /// Print the list of config locations to search for.
    PrintLoc,

    /// Print the example of a configuration file content.
    Example,
}

#[derive(Debug, Clone, Args)]
pub(crate) struct TagArgs {
    #[command(subcommand)]
    pub command: TagCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub(crate) enum TagCommand {
    /// Extract all tags for every media file in the photos' collection.
    List,

    /// Extract date and time information
    GetTime {
        #[arg(long_help = r#"How to parse the string into datetime.

Examples:
- IMG_%Y%m%d_%H%M%S.jpg
- VID_%Y%m%d_%H%M%S.mp4
- Ascii(["%Y:%m:%d %T"]) (for the EXIF tags)
- %Y-%m-%dT%H:%M:%S%.6f%Z (ISO-8601)
- %+ (the same as the previous example)"#)]
        format: String,

        #[arg(
            short,
            long,
            long_help = r#"The metadata tag will be used as the source.

Examples:
- 'Date and time of original data generation' (EXIF tag);
- 'creation_time' (mp4 tag);

If the tag is missing, the file name will be used as the source."#
        )]
        tag: Option<String>,
    },
}
