use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    args: PngMeArgs,
}
#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
#[derive(Args)]
pub struct EncodeArgs {
    chunk_type: String,
    message: String,
    file_path: PathBuf,
    output_file: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodeArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    file_path: PathBuf,
}
