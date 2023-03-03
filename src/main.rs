mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use crate::args::PngMeArgs;
use crate::commands::{decode, encode, print_chunks, remove};
use clap::Parser;

#[derive(Parser)]
#[command(name = "Secret Messages")]
#[command(author = "Mushroom <mushroom020@proton.me>")]
#[command(version = "1.0")]
#[command(about = "encodes and decodes secret messages into the png files", long_about = None)]
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
