use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum PngMeArgs {
    /// Encode Message to the given file or the file you provide
    Encode(EncodeArgs),
    /// Decode message
    Decode(DecodeArgs),
    /// Remove message
    Remove(RemoveArgs),
    /// Print all the messages
    Print(PrintArgs),
}
#[derive(Args)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
