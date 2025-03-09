// src/args/mod.rs

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
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    #[clap(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Parser)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Parser)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}