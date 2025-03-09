use clap::Parser;
use pngspy::args::Cli;
use pngspy::commands::run;
use pngspy::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}