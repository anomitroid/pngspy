pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub mod args;
pub mod commands;
pub mod chunks;
pub mod utils;
pub mod png;
pub mod error;

pub use commands::run;