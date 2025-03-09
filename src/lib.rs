pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub mod args;
pub mod commands;
pub mod chunk_type;
pub mod chunk;
pub mod crc;
pub mod png;

pub use commands::run;