use std::cmp::Reverse;

use clap::Parser as _;

use foton::{get_tag_values_distribution, get_tags_distribution, Library};

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
                    for f in lib.iter(list_resource) {
                        println!("{}", f.display());
                    }
                } else {
                    for f in lib.iter_all() {
                        println!("{}", f.display());
                    }
                }
            } else {
                fallback_config_not_found();
            }
        }
        Command::Config(ca) => match ca.command {
            ConfigCommand::Print => {
                if let Some(config) = config {
                    println!("{}", config);
                } else {
                    fallback_config_not_found();
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
                        let all_tags = get_tags_distribution(lib.iter_all());
                        let mut all_tags: Vec<_> = all_tags.into_iter().collect();
                        // FIXME: do not .clone()
                        all_tags
                            .sort_unstable_by_key(|(key, count)| (Reverse(*count), key.clone()));
                        for (k, count) in all_tags {
                            println!("{:6} {}", count, k);
                        }
                    }
                    TagCommand::Group { tag_name } => {
                        let all_tags = get_tag_values_distribution(&tag_name, lib.iter_all());
                        let mut all_tags: Vec<_> = all_tags.into_iter().collect();
                        // FIXME: do not .clone()
                        all_tags
                            .sort_unstable_by_key(|(val, count)| (Reverse(*count), val.clone()));
                        for (k, count) in all_tags {
                            println!("{:6} {}", count, k);
                        }
                    }
                }
            } else {
                fallback_config_not_found();
            }
        }
    }

    Ok(())
}

fn fallback_config_not_found() {
    let locations: Vec<_> = Config::locations().collect();

    eprintln!("Not found config file in any of the locations");
    for loc in &locations {
        eprintln!(" - {}", loc.display());
    }

    eprintln!("To continue please create a file in any of the locations above.");
    let example = if let Some(p) = locations.last() {
        eprintln!();
        eprintln!("cat <<EOF > {}", p.display());
        true
    } else {
        println!();
        println!("```");
        false
    };

    println!("{}", Config::stub());

    if example {
        eprintln!("EOF");
    } else {
        println!("```");
    }
}
