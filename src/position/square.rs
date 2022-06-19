#[derive(Debug)]
pub struct Square {
    file: usize,
    rank: usize,
}

impl Square {
    //TODO: Proper error handling with anyhow
    pub fn from_str(s: &str) -> Option<Self> {
        let rank: char;
        let file: char;

        if let Some(c) = s.chars().nth(0) {
            file = c;
        } else {
            return None;
        }

        if let Some(c) = s.chars().nth(1) {
            rank = c;
        } else {
            return None;
        }

        if (rank as usize) < 0x39 && (rank as usize) > 0x30 {
            if (file as usize) < 0x69 && (file as usize) > 0x60 {
                return Some(Square {
                    file: (file as usize) - ('a' as usize),
                    rank: (rank as usize) - ('1' as usize),
                });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert!(Square::from_str("a8").is_some());
        assert!(Square::from_str("a9").is_none());
        assert!(Square::from_str("").is_none());
    }
}
