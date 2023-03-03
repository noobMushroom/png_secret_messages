use crate::{Error, Result};
/// png file always start with signature
/// first eight byte of png type always contain these eight bytes  137 80 78 71 13 10 26 10
/// each chunk consist of four parts
use std::convert::TryFrom;
use std::fmt;
use std::str;
use std::str::FromStr;
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkType(pub [u8; 4]); //tuple type stuct a valid type chunk is equal to 4 bytes

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    pub fn is_critical(&self) -> bool {
        let [a, ..] = self.0;
        if a.is_ascii_uppercase() {
            return true;
        }
        false
    }

    pub fn is_public(&self) -> bool {
        let [_, b, ..] = self.0;
        if b.is_ascii_uppercase() {
            return true;
        }
        false
    }

    #[allow(dead_code)]
    pub fn is_reserved_bit_valid(&self) -> bool {
        let [_, _, c, _] = self.0;
        if c.is_ascii_uppercase() {
            return true;
        }
        false
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let [.., d] = self.0;
        if d.is_ascii_uppercase() {
            return false;
        }
        true
    }

    #[allow(dead_code)]
    pub fn is_valid_byte(byte: u8) -> bool {
        if byte.is_ascii_alphabetic() {
            return true;
        }
        false
    }
    pub fn to_string(&self) -> String {
        str::from_utf8(&self.0).unwrap().to_owned()
    }

    pub fn is_valid(&self) -> bool {
        let [_, _, c, _] = self.0;
        if c.is_ascii_uppercase() {
            return true;
        }
        false
    }
}
impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        for i in bytes {
            if !i.is_ascii_alphabetic() {
                return Err(Error::from("not a valid string"));
            }
        }
        let chunk_array: [u8; 4] = bytes
            .try_into()
            .map_err(|_| Error::from("not a valid string"))?;
        Ok(ChunkType(chunk_array))
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        let chunk = ChunkType(bytes);
        match chunk.is_valid() {
            true => Ok(chunk),
            false => Err(Error::from("bytes is invalid bytes")),
        }
    }
}
impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }
    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
