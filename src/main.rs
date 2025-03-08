mod args;
mod commands;

use clap::Parser;
use args::Cli;
use commands::run;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}