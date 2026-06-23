use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum error {
    #[error("Usage: first_rust_scanner <example.com>")]
    CliUsage,
}



