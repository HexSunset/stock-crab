#![allow(dead_code)]

use super::*;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    pub fn from_char(c: char) -> Result<(Self, Color)> {
        use PieceType as Ptype;

        let color = match c.is_lowercase() {
            true => Color::Black,
            false => Color::White,
        };
        let c = c.to_lowercase().next().unwrap();
        match c {
            'k' => Ok((Ptype::King, color)),
            'q' => Ok((Ptype::Queen, color)),
            'r' => Ok((Ptype::Rook, color)),
            'b' => Ok((Ptype::Bishop, color)),
            'n' => Ok((Ptype::Knight, color)),
            'p' => Ok((Ptype::Pawn, color)),
            _ => Err(anyhow!("Invalid character '{c}'")),
        }
    }

    pub fn to_char(&self, c: Color) -> char {
        let out = match self {
            PieceType::King => 'k',
            PieceType::Queen => 'q',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            PieceType::Pawn => 'p',
        };

        match c {
            Color::White => out.to_ascii_uppercase(),
            Color::Black => out.to_ascii_lowercase(),
        }
    }
}

pub fn get_piece_attack_map(
    p: PieceType,
    c: Color,
    b: &BitBoard,
    friendly: &BitBoard,
    opposing: &BitBoard,
) -> BitBoard {
    match p {
        PieceType::King => get_king_attack_map(b, friendly),
        PieceType::Queen => get_queen_attack_map(b, friendly, opposing),
        PieceType::Rook => get_rook_attack_map(b, friendly, opposing),
        PieceType::Bishop => get_bishop_attack_map(b, friendly, opposing),
        PieceType::Knight => get_knight_attack_map(b, friendly, opposing),
        PieceType::Pawn => get_pawn_attack_map(b, c),
    }
}

//TODO
fn get_pawn_attack_map(b: &BitBoard, c: Color) -> BitBoard {
    let mut out = BitBoard::new();

    for file in 0..8 {
        for rank in 0..8 {
            if b.get(file, rank) == Some(true) {
                match c {
                    Color::Black => {
                        if file < 7 {
                            out.set(file + 1, rank - 1);
                        }
                        if file > 0 {
                            out.set(file - 1, rank - 1);
                        }
                    }
                    Color::White => {
                        if file < 7 {
                            out.set(file + 1, rank + 1);
                        }
                        if file > 0 {
                            out.set(file - 1, rank + 1);
                        }
                    }
                }
            }
        }
    }

    out
}

fn get_knight_attack_map(b: &BitBoard, friendly: &BitBoard, opposing: &BitBoard) -> BitBoard {
    let mut out = BitBoard::new();

    for file in 0..8 {
        for rank in 0..8 {
            if b.get(file, rank) == Some(true) {
                // file - 1, rank + 2
                if rank < 6 && file > 0 {
                    out.set(file - 1, rank + 2);
                }

                // file - 1, rank - 2
                if rank > 1 && file > 0 {
                    out.set(file - 1, rank - 2);
                }

                // file + 1, rank + 2
                if rank < 6 && file < 7 {
                    out.set(file + 1, rank + 2);
                }

                // file + 1, rank - 2
                if rank > 1 && file < 7 {
                    out.set(file + 1, rank - 2);
                }

                // file - 2, rank - 1
                if rank > 0 && file > 1 {
                    out.set(file - 2, rank - 1);
                }

                // file - 2, rank + 1
                if rank < 7 && file > 1 {
                    out.set(file - 2, rank + 1);
                }

                // file + 2, rank + 1
                if file < 6 && rank < 7 {
                    out.set(file + 2, rank + 1);
                }

                // file + 2, rank - 1
                if file < 6 && rank > 0 {
                    out.set(file + 2, rank - 1);
                }
            }
        }
    }

    out
}

fn get_bishop_attack_map(b: &BitBoard, friendly: &BitBoard, opposing: &BitBoard) -> BitBoard {
    use std::cmp;

    let mut out = BitBoard::new();

    for file in 0..8 {
        for rank in 0..8 {
            if b.get(file, rank) == Some(true) {
                // Diagonally toward h8 corner
                for i in 1..8 - cmp::max(rank, file) {
                    out.set(file + i, rank + i);

                    if friendly.get(file + i, rank + i) == Some(true) {
                        break;
                    } else if opposing.get(file + i, rank + i) == Some(true) {
                        break;
                    }
                }

                // Diagonally toward a8 corner
                for i in 1..=cmp::min(file, 7 - rank) {
                    out.set(file - i, rank + i);

                    if friendly.get(file - i, rank + i) == Some(true) {
                        break;
                    } else if opposing.get(file - i, rank + i) == Some(true) {
                        break;
                    }
                }

                // Diagonally toward a1 corner
                for i in 1..=cmp::min(file, rank) {
                    out.set(file - i, rank - i);

                    if friendly.get(file - i, rank - i) == Some(true) {
                        break;
                    } else if opposing.get(file - i, rank - i) == Some(true) {
                        break;
                    }
                }

                // Diagonally toward h8 corner
                for i in 1..=cmp::min(7 - file, rank) {
                    out.set(file + i, rank - i);

                    if friendly.get(file + i, rank - i) == Some(true) {
                        break;
                    } else if opposing.get(file + i, rank - i) == Some(true) {
                        break;
                    }
                }
            }
        }
    }

    out
}

fn get_rook_attack_map(b: &BitBoard, friendly: &BitBoard, opposing: &BitBoard) -> BitBoard {
    let mut out = BitBoard::new();

    for file in 0..8 {
        for rank in 0..8 {
            if b.get(file, rank) == Some(true) {
                // Starting the search from next to our piece so it stops at the right square
                for x in (0..file).rev() {
                    out.set(x, rank);

                    if friendly.get(x, rank) == Some(true) {
                        break;
                    } else if opposing.get(x, rank) == Some(true) {
                        break;
                    }
                }

                for x in (file + 1)..8 {
                    out.set(x, rank);

                    if friendly.get(x, rank) == Some(true) {
                        break;
                    } else if opposing.get(x, rank) == Some(true) {
                        break;
                    }
                }

                for y in (0..rank).rev() {
                    out.set(file, y);

                    if friendly.get(file, y) == Some(true) {
                        break;
                    } else if opposing.get(file, y) == Some(true) {
                        out.set(file, y);
                        break;
                    }
                }

                for y in (rank + 1)..8 {
                    out.set(file, y);

                    if friendly.get(file, y) == Some(true) {
                        break;
                    } else if opposing.get(file, y) == Some(true) {
                        break;
                    }
                }
            }
        }
    }

    out
}

fn get_queen_attack_map(b: &BitBoard, friendly: &BitBoard, opposing: &BitBoard) -> BitBoard {
    let mut out = BitBoard::new();

    out.or_assign(get_bishop_attack_map(b, friendly, opposing));
    out.or_assign(get_rook_attack_map(b, friendly, opposing));

    out
}

fn get_king_attack_map(b: &BitBoard, friendly: &BitBoard) -> BitBoard {
    let mut out = BitBoard::new();

    // Find location of king
    for file in 0..8 {
        for rank in 0..8 {
            if b.get(file, rank) == Some(true) {
                // loop over all the surrounding squares
                let f_min = if file > 0 { file - 1 } else { file };
                let f_max = if file < 7 { file + 1 } else { file };

                let r_min = if rank > 0 { rank - 1 } else { rank };
                let r_max = if rank < 7 { rank + 1 } else { rank };
                for f in f_min..=f_max {
                    for r in r_min..=r_max {
                        out.set(f, r);
                    }
                }
            }
        }
    }

    out
}

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_piece_attack_map() {
        king_attack_map();
        rook_attack_map();
        bishop_attack_map();
        queen_attack_map();
        knight_attack_map();
        pawn_attack_map();
    }

    fn knight_attack_map() {
        let pos = parse::from_fen("8/8/8/8/k1q1N3/8/8/3K4 w - - 0 1".to_string()).unwrap();
        assert!(pos.w_attacks.get_board(PieceType::Knight).as_u64() == 44272527353856);

        let pos = parse::from_fen("8/8/8/8/k1q5/8/1N6/3K4 w - - 0 1".to_string()).unwrap();
        assert!(pos.w_attacks.get_board(PieceType::Knight).as_u64() == 84410376);
    }

    fn king_attack_map() {
        let pos = parse::from_fen("8/8/8/3K4/8/6k1/8/8 w - - 0 1".to_string()).unwrap();
        assert!(pos.w_attacks_all.as_u64() == 30907054424064);

        let pos = parse::from_fen("K7/8/8/8/8/8/8/7k w - - 0 1".to_string()).unwrap();
        assert!(pos.w_attacks.get_board(PieceType::King).as_u64() == 217017207043915776);

        let pos = parse::from_fen("KR6/RR6/8/8/8/8/8/7k w - - 0 1".to_string()).unwrap();
        assert!(pos.w_attacks.get_board(PieceType::King).as_u64() == 217017207043915776);
    }

    fn rook_attack_map() {
        let pos = parse::from_fen("7K/2B2R2/8/8/8/5k2/8/8 w - - 0 1".to_string()).unwrap();
        assert!(pos.w_attacks.get_board(PieceType::Rook).as_u64() == 2367802826440048640);
    }

    fn bishop_attack_map() {
        let pos = parse::from_fen("8/3k4/4B3/8/8/7K/8/8 w - - 0 1".to_string()).unwrap();
        assert!(pos.w_attacks.get_board(PieceType::Bishop).as_u64() == 4622945190443876608);
    }

    fn queen_attack_map() {
        let pos = parse::from_fen("8/8/1k2Q3/8/2N5/4K3/8/8 w - - 0 1".to_string()).unwrap();
        assert!(pos.w_attacks.get_board(PieceType::Queen).as_u64() == 6068862423586045952);
    }

    fn pawn_attack_map() {
        let pos = parse::from_fen("2K2k2/1P1P1P2/5p2/8/8/8/8/8 w - - 0 1".to_string()).unwrap();

        assert!(pos.w_attacks.get_board(PieceType::Pawn).as_u64() == 6124895493223874560);
        assert!(pos.b_attacks.get_board(PieceType::Pawn).as_u64() == 343597383680);
    }
}
