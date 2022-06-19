#![allow(dead_code)]

pub struct BitBoard(u64);

impl BitBoard {
    fn toggle(&mut self, x: usize, y: usize) -> Result<(), ()> {
        if y <= 7 && x <= 7 {
            let mask: u64 = 1 << (y * 8 + x);
            self.0 ^= mask;
            Ok(())
        } else {
            Err(())
        }
    }

    fn get(&self, x: usize, y: usize) -> Result<bool, ()> {
        if x <= 7 && y <= 7 {
            let mask: u64 = 1 << (y * 8 + x);
            Ok((self.0 | mask) != 0)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bitboard() {
        let mut bb1 = BitBoard(0);
        assert!(bb1.toggle(0, 0).is_ok());
        assert!(bb1.0 == 1);

        assert!(bb1.toggle(0, 0).is_ok());
        assert!(bb1.0 == 0);

        assert!(bb1.toggle(8, 0).is_err());

        assert!(bb1.toggle(7, 7).is_ok());
        assert!(bb1.0 == 9223372036854775808);

        assert!(bb1.toggle(7, 7).is_ok());
        assert!(bb1.0 == 0);
    }
}
