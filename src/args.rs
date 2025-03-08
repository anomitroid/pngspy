use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: PngSpyArgs,
}

#[derive(Debug, Subcommand)]
pub enum PngSpyArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs)
}

#[derive(Debug, Parser)]
pub struct EncodeArgs {
    #[clap(short, long)]
    pub input: PathBuf,

    #[clap(short, long)]
    pub message: String,

    #[clap(short, long)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct DecodeArgs {
    #[clap(short, long)]
    pub input: PathBuf,

    #[clap(short, long, default_value = "decoded_message.txt")]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct RemoveArgs {
    #[clap(short, long)]
    pub input: PathBuf,

    #[clap(short, long)]
    pub backup: bool,
}

#[derive(Debug, Parser)]
pub struct PrintArgs {
    #[clap(short, long)]
    pub input: PathBuf,

    #[clap(short, long)]
    pub verbose: bool,
}