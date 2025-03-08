mod args;
mod commands;

use clap::Parser;
use args::{Cli, PngSpyArgs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        PngSpyArgs::Encode(args) => commands::handle_encode(args),
        PngSpyArgs::Decode(args) => commands::handle_decode(args),
        PngSpyArgs::Remove(args) => commands::handle_remove(args),
        PngSpyArgs::Print(args) => commands::handle_print(args),
    }
}