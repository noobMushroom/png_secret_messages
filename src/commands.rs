use std::convert::TryFrom;
use std::fs;
use std::io::Write;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::{Error, Result};
use anyhow::anyhow;
use std::path::{Path, PathBuf};
/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    //creating file using from file method available in png struct
    let mut png = Png::from_file(&args.file_path)?;
    // creating chunk type because Chunk take chunk type
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    // creating new chunk by the data use provided
    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    // adding our secret chunk to the png chunk
    png.append_chunk(chunk);
    let file_name = args.file_path.file_name().unwrap();
    // handling output path if user provided any
    let mut output_file = match args.output_file {
        Some(path) => path.join(file_name),
        None => PathBuf::from(file_name),
    };
    // checking if the file already exits if yes then adding number at then end of it
    let mut suffix = 1;
    while output_file.exists() {
        let file_stem = output_file
            .file_stem()
            .ok_or(anyhow!("failed to create file"))?;
        let file_name = format!("{}{}.png", file_stem.to_string_lossy(), suffix);
        output_file.set_file_name(file_name);
        suffix += 1
    }
    let mut file = fs::File::create(output_file)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}
/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(args.file_path)?;
    let message = png.chunk_by_type(&args.chunk_type);
    match message {
        Some(data) => {
            println!(
                "Chunk: {} \nMessage: {:?}",
                data.chunk_type.to_string(),
                data.data_as_string().unwrap()
            );
            Ok(())
        }
        None => Err(Error::from("chunk not found")),
    }
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    png.remove_chunk(&args.chunk_type as &str)?;
    let mut file = fs::File::open(args.file_path)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    todo!()
}
