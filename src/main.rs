mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use crate::args::PngMeArgs;
use crate::commands::{decode, encode, print_chunks, remove};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    pub args: PngMeArgs,
}
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.args {
        PngMeArgs::Encode(args) => encode(args)?,
        PngMeArgs::Decode(args) => decode(args)?,
        PngMeArgs::Remove(args) => remove(args)?,
        PngMeArgs::Print(args) => print_chunks(args)?,
    }
    Ok(())
}
