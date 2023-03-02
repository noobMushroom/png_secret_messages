// chnunk struct consist of 4 things 4bytes length and 4 bytes chunk type and chunk data that can
// be variable leangth and finally crc

use crate::chunk_type::ChunkType;
use crate::{Error, Result};
use crc::{Crc, CRC_32_MEF};
use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};
use std::str;
const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_MEF);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
    pub crc: u32,
}
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let concated = [chunk_type.bytes().as_ref(), data.as_slice()].concat();
        let checksum = CRC.checksum(&concated);
        Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            crc: checksum,
        }
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn chunk_type(&self) -> ChunkType {
        self.chunk_type
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        match str::from_utf8(&self.data[..]) {
            Ok(data) => Ok(data.to_string()),
            Err(_) => Err(Error::from("invalid string")),
        }
    }
}

fn calculate_from_bytes(bytes: &[u8]) -> u32 {
    let mut reader = BufReader::new(bytes);
    let mut buffer = [0_u8; 4];
    reader.read_exact(&mut buffer).unwrap();
    u32::from_be_bytes(buffer)
}
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let crc = &bytes[bytes.len() - 4..];
        let original_crc = calculate_from_bytes(crc); // crc provided
        let checksum = CRC.checksum(&bytes[4..bytes.len() - 4]);
        if original_crc == checksum {
            let length = calculate_from_bytes(&bytes[0..4]);
            let data = bytes[8..bytes.len() - 4].to_vec();
            let chunk_type = &bytes[4..8];
            let chunk_array: [u8; 4] = chunk_type.try_into().unwrap();
            Ok(Chunk {
                length,
                chunk_type: ChunkType(chunk_array),
                data,
                crc: checksum,
            })
        } else {
            Err(Error::from("invalid chunk"))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 1432711019;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 1432711019);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 1432711019);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 1432711019;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 1432711019);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 1432711019;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
