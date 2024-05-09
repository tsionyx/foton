use clap::{Args, Parser, Subcommand};

use foton::MediaType;

#[derive(Debug, Clone, Parser)]
/// Manage the photos and videos collection.
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// List files in a collection.
    List {
        /// Type of the resource to find.
        type_: Option<MediaType>,
    },

    /// View or create a configuration file.
    Config(ConfigArgs),

    /// Show metadata.
    Tags(TagArgs),
}

#[derive(Debug, Copy, Clone, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Debug, Copy, Clone, Subcommand)]
pub enum ConfigCommand {
    /// Print the current configuration file in use (if any).
    Print,

    /// Print the list of config locations to search for.
    PrintLoc,

    /// Print the example of a configuration file content.
    Example,
}

#[derive(Debug, Clone, Args)]
pub struct TagArgs {
    #[command(subcommand)]
    pub command: TagCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub(crate) enum TagCommand {
    /// Extract all tags for every media file in the photos' collection.
    List,
}
