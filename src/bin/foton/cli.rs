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
}
