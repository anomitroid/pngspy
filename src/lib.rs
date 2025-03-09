// src/lib.rs

pub mod args;
pub mod commands;
pub mod chunks;
pub mod utils;
pub mod png;
pub mod error;
pub mod network;

pub use commands::run;

pub type Result<T> = error::Result<T>;