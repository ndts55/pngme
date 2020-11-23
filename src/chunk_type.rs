use std::cmp::{Eq, PartialEq};
use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn new(bytes: [u8; 4]) -> Self {
        ChunkType { bytes }
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        if bytes.iter().any(|byte| !byte.is_ascii_alphabetic()) {
            Err("All values need to be in a-zA-Z")
        } else {
            Ok(ChunkType { bytes })
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("s should be 4 characters long");
        }

        if s.chars().any(|ch| !ch.is_ascii_alphabetic()) {
            return Err("All values need to be in a-zA-Z");
        }

        let mut bytes: [u8; 4] = [0, 0, 0, 0];
        for (idx, ch) in s.char_indices() {
            bytes[idx] = ch as u8;
        }

        Ok(ChunkType { bytes })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.bytes.to_vec()).unwrap())
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
        assert!(chunk.bytes[0].is_ascii_uppercase());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.bytes[0].is_ascii_uppercase());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.bytes[1].is_ascii_uppercase());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.bytes[1].is_ascii_uppercase());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.bytes[2].is_ascii_uppercase());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.bytes[2].is_ascii_uppercase());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.bytes[3].is_ascii_lowercase());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.bytes[3].is_ascii_lowercase());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.bytes[2].is_ascii_uppercase());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.bytes[2].is_ascii_uppercase());

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
