#![allow(dead_code)]

use anyhow::{anyhow, Result};

///```
///#[derive(Debug, Clone, PartialEq)]
///pub struct Square {
///    file: usize,
///    rank: usize,
///}
///```
#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    pub file: u32,
    pub rank: u32,
}

impl Square {
    //TODO: Proper error handling with anyhow
    pub fn from_str(s: &str) -> Result<Self> {
        let rank: char;
        let file: char;

        if let Some(c) = s.chars().nth(0) {
            file = c;
        } else {
            return Err(anyhow!("Square string '{s}' is too short"));
        }

        if let Some(c) = s.chars().nth(1) {
            rank = c;
        } else {
            return Err(anyhow!("Square string '{s}' is too short"));
        }

        if (rank as usize) < 0x39 && (rank as usize) > 0x30 {
            if (file as usize) < 0x69 && (file as usize) > 0x60 {
                return Ok(Square {
                    file: (file as u32) - ('a' as u32),
                    rank: (rank as u32) - ('1' as u32),
                });
            }
        }

        Err(anyhow!("Invalid square '{s}'"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert!(Square::from_str("a8").is_ok());
        assert!(Square::from_str("a9").is_err());
        assert!(Square::from_str("").is_err());
    }
}
