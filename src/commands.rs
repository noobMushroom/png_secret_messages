use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::{Error, Result};
use std::fs;
use std::io::Write;
use std::str::FromStr;
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
    let _file_name = args.file_path.file_name().unwrap();
    // handling output path if user provided any
    let mut file = match &args.output_file {
        Some(path) => {
            if path.exists() {
                return Err(Error::from("file already exists"));
            } else {
                fs::File::create(path)?
            }
        }
        None => fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&args.file_path)?,
    };
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
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&args.file_path)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    todo!()
}
