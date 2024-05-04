mod app;
mod cli;
mod config;

type AnyError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), AnyError> {
    env_logger::init();
    app::run()
}
