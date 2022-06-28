#![allow(dead_code)]

use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitBoard(u64);

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for y in 0..8 {
            for x in 0..8 {
                match self.get(x, 7 - y) {
                    Some(true) => output.push_str("1"),
                    Some(false) => output.push_str("0"),
                    None => (),
                }
            }
            output.push_str("\n");
        }

        write!(f, "{}", output)
    }
}

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard(0)
    }

    pub fn from(n: u64) -> BitBoard {
        BitBoard(n)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn as_bitvec(&self) -> Vec<usize> {
        let mut out = vec![0; 64];

        let linear_index = |x, y| (y * 8 + x) as usize;

        for x in 0..8 {
            for y in 0..8 {
                if self.get(x, y) == Some(true) {
                    out[linear_index(x, y)] = 1;
                }
            }
        }

        out
    }

    // Set square on
    pub fn set(&mut self, x: u32, y: u32) {
        if y <= 7 && x <= 7 {
            let mask: u64 = 1 << (y * 8 + x);
            self.0 |= mask;
        }
    }

    // Set square off
    pub fn unset(&mut self, x: u32, y: u32) {
        if y <= 7 && x <= 7 {
            match self.get(x, y) {
                Some(false) => (),
                Some(true) => self.toggle(x, y),
                None => (),
            }
        }
    }

    // Toggle state of square
    pub fn toggle(&mut self, x: u32, y: u32) {
        if y <= 7 && x <= 7 {
            let mask: u64 = 1 << (y * 8 + x);
            self.0 ^= mask;
        } else {
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<bool> {
        if x <= 7 && y <= 7 {
            let mask: u64 = 1 << (y * 8 + x);
            Some((self.0 & mask) != 0)
        } else {
            None
        }
    }

    pub fn or_assign(&mut self, b: BitBoard) {
        self.0 |= b.as_u64();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bitboard() {
        let mut bb1 = BitBoard(0);
        bb1.toggle(0, 0);
        assert!(bb1.as_u64() == 1);

        bb1.toggle(0, 0);
        assert!(bb1.as_u64() == 0);

        bb1.toggle(8, 0);
        assert!(bb1.get(8, 0) == None);

        bb1.toggle(7, 7);
        assert!(bb1.get(7, 7) == Some(true));
        assert!(bb1.as_u64() == 9223372036854775808);

        bb1.toggle(7, 7);
        assert!(bb1.get(7, 7) == Some(false));
        assert!(bb1.as_u64() == 0);

        bb1.unset(7, 7);
        assert!(bb1.as_u64() == 0);

        bb1.set(7, 7);
        assert!(bb1.as_u64() != 0);

        bb1.set(7, 7);
        assert!(bb1.as_u64() != 0);
    }
}
