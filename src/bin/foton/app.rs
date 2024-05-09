use std::borrow::Cow;

use clap::Parser as _;
use log::warn;

use foton::{Library, TimeFormat, TimeSource};

use crate::{
    cli::{Cli, Command, ConfigCommand, TagCommand},
    config::Config,
};

type AnyError = Box<dyn std::error::Error + Send + Sync>;

pub fn run() -> Result<(), AnyError> {
    let config = Config::load()?;
    let cli = Cli::parse();

    match cli.command {
        Command::List { type_ } => {
            if let Some(config) = config {
                let lib = Library::with_paths(config.library);
                if let Some(list_resource) = type_ {
                    for f in lib.iter(list_resource.into()) {
                        println!("{}", f);
                    }
                } else {
                    for f in lib.iter_all() {
                        println!("{}", f);
                    }
                }
            } else {
                fallback_config_not_found()?;
            }
        }
        Command::Config(ca) => match ca.command {
            ConfigCommand::Print => {
                if let Some(config) = config {
                    println!("{}", config);
                } else {
                    fallback_config_not_found()?;
                }
            }
            ConfigCommand::PrintLoc => {
                for loc in Config::locations() {
                    let exists_sign = if loc.exists() { "[X]" } else { "[ ]" };
                    println!(" {} {}", exists_sign, loc.display());
                }
            }
            ConfigCommand::Example => {
                println!("{}", Config::stub());
            }
        },
        Command::Tags(ta) => {
            if let Some(config) = config {
                let lib = Library::with_paths(config.library);
                match ta.command {
                    TagCommand::List => {
                        for resource in lib.iter_all() {
                            match resource.get_tags() {
                                Ok(map) => {
                                    println!("--- {} ---", resource);
                                    for (k, v) in map {
                                        println!("{}: {}", k, v);
                                    }
                                    println!();
                                }
                                Err(err) => {
                                    warn!("{}: {:?}", resource, err);
                                }
                            }
                        }
                    }
                    TagCommand::GetTime { format, tag } => {
                        let sources = if let Some(format) = format {
                            let format = TimeFormat(format);
                            let source = if let Some(name) = tag {
                                TimeSource::Tag { name, format }
                            } else {
                                TimeSource::FileName { format }
                            };
                            Cow::Owned(vec![source])
                        } else {
                            config
                                .metadata
                                .as_ref()
                                .map(|md| Cow::Borrowed(&md.time_source))
                                .unwrap_or_default()
                        };
                        if sources.is_empty() {
                            return Err(
                                "Either specify --format or add metadata.time_sources into config"
                                    .into(),
                            );
                        } else {
                            for f in lib.iter_all() {
                                if let Some(time) = f.get_datetime(&sources) {
                                    println!("{}: {:?}", f, time);
                                } else {
                                    println!("{}: UNDEFINED", f);
                                }
                            }
                        }
                    }
                }
            } else {
                fallback_config_not_found()?;
            }
        }
    }

    Ok(())
}

fn fallback_config_not_found() -> Result<(), AnyError> {
    use std::fmt::Write as _;

    let locations: Vec<_> = Config::locations().collect();

    let mut err = String::new();
    writeln!(err, "Not found config file in any of the locations")?;
    for loc in &locations {
        writeln!(err, " - {}", loc.display())?;
    }

    writeln!(
        err,
        "To continue please create a file in any of the locations above."
    )?;

    writeln!(err)?;
    writeln!(err, "# Example")?;
    let example = if let Some(p) = locations.last() {
        writeln!(err, "cat <<EOF > {}", p.display())?;
        true
    } else {
        writeln!(err, "```")?;
        false
    };

    writeln!(err, "{}", Config::stub())?;

    if example {
        write!(err, "EOF")?;
    } else {
        write!(err, "```")?;
    }

    Err(err.into())
}
