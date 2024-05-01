use clap::Parser as _;

use crate::{
    cli::{Cli, Command, ConfigCommand},
    config::Config,
    dir::Library,
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
