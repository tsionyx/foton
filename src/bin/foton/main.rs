use std::process::exit;

mod app;
mod cli;
mod config;

type AnyError = Box<dyn std::error::Error + Send + Sync>;

fn main() {
    env_logger::init();
    if let Err(err) = app::run() {
        eprintln!("Error: {}", err);
        exit(1)
    }
}
