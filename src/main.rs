//! Simple photos and videos management tool.
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![forbid(unsafe_code)]

mod app;
mod cli;
mod config;
pub mod dir;
mod file_types;

type AnyError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), AnyError> {
    app::run()
}
