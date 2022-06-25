#![allow(dead_code)]

mod bitboard;
mod movegen;
mod piece;
mod square;

use bitboard::BitBoard;
use piece::{Color, PieceType};
use square::Square;
use std::collections::HashMap;

use anyhow::{anyhow, Result};

#[derive(Clone, Debug, PartialEq)]
pub struct Castling {
    king_side: bool,
    queen_side: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SideMap(HashMap<PieceType, BitBoard>);

impl SideMap {
    pub fn new() -> SideMap {
        SideMap(HashMap::from([
            (PieceType::King, BitBoard::new()),
            (PieceType::Queen, BitBoard::new()),
            (PieceType::Rook, BitBoard::new()),
            (PieceType::Bishop, BitBoard::new()),
            (PieceType::Knight, BitBoard::new()),
            (PieceType::Pawn, BitBoard::new()),
        ]))
    }

    pub fn toggle(&mut self, ptype: PieceType, file: u32, rank: u32) {
        self.0.entry(ptype).and_modify(|b| b.toggle(file, rank));
    }

    pub fn get(&self, ptype: PieceType, file: u32, rank: u32) -> Option<bool> {
        self.0.get(&ptype).unwrap().get(file, rank)
    }

    pub fn set(&mut self, ptype: PieceType, file: u32, rank: u32) {
        self.0.get_mut(&ptype).unwrap().set(file, rank)
    }

    pub fn unset(&mut self, ptype: PieceType, file: u32, rank: u32) {
        self.0.get_mut(&ptype).unwrap().unset(file, rank)
    }

    pub fn combine(&self) -> BitBoard {
        let mut out = BitBoard::new();
        for (_, pieceboard) in &self.0 {
            out.or_assign(*pieceboard);
        }

        out
    }

    pub fn get_map(&self) -> &HashMap<PieceType, BitBoard> {
        &self.0
    }

    pub fn get_mut_map(&mut self) -> &mut HashMap<PieceType, BitBoard> {
        &mut self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub side: Color,
    pub halfturn: usize,

    pub w_castling: Castling,

    pub b_castling: Castling,

    pub en_passant: Option<Square>,

    pub w_pieces_all: BitBoard,
    pub w_pieces: SideMap,

    pub b_pieces_all: BitBoard,
    pub b_pieces: SideMap,

    pub history: Vec<Move>,
    pub legal_moves: Vec<Move>,
}

impl Position {
    /// Assumes that m is a legal move.
    /// Will not do any checking.
    fn change_board(&mut self, m: Move) {
        let x_from = m.from.file;
        let y_from = m.from.rank;

        let x_to = m.to.file;
        let y_to = m.to.rank;

        let sidemap = match self.side {
            Color::White => &mut self.w_pieces,
            Color::Black => &mut self.b_pieces,
        };

        let all_board = match self.side {
            Color::White => &mut self.w_pieces_all,
            Color::Black => &mut self.b_pieces_all,
        };

        all_board.unset(x_from, y_from);
        all_board.set(x_to, y_to);
        sidemap
            .get_mut_map()
            .get_mut(&m.ptype)
            .unwrap()
            .unset(x_from, y_from);

        sidemap
            .get_mut_map()
            .get_mut(&m.ptype)
            .unwrap()
            .set(x_to, y_to);

        if let Some(piece) = &m.change.captured {
            let opp_sidemap = match self.side {
                Color::White => &mut self.b_pieces,
                Color::Black => &mut self.w_pieces,
            };

            let opp_all_board = match self.side {
                Color::White => &mut self.b_pieces_all,
                Color::Black => &mut self.w_pieces_all,
            };

            opp_all_board.unset(x_to, y_to);

            opp_sidemap
                .get_mut_map()
                .get_mut(&piece)
                .unwrap()
                .unset(x_to, y_to);
        };

        self.history.push(m);
    }

    fn reverse_last_change(&mut self) {
        let m = self.history.pop().unwrap();

        let x_from = m.from.file;
        let y_from = m.from.rank;

        let x_to = m.to.file;
        let y_to = m.to.rank;

        let sidemap = match self.side {
            Color::White => &mut self.w_pieces,
            Color::Black => &mut self.b_pieces,
        };

        let all_board = match self.side {
            Color::White => &mut self.w_pieces_all,
            Color::Black => &mut self.b_pieces_all,
        };

        all_board.set(x_from, y_from);
        all_board.unset(x_to, y_to);
        sidemap
            .get_mut_map()
            .get_mut(&m.ptype)
            .unwrap()
            .set(x_from, y_from);

        sidemap
            .get_mut_map()
            .get_mut(&m.ptype)
            .unwrap()
            .unset(x_to, y_to);

        if let Some(piece) = &m.change.captured {
            let opp_sidemap = match self.side {
                Color::White => &mut self.b_pieces,
                Color::Black => &mut self.w_pieces,
            };

            let opp_all_board = match self.side {
                Color::White => &mut self.b_pieces_all,
                Color::Black => &mut self.w_pieces_all,
            };

            opp_all_board.set(x_to, y_to);
            opp_sidemap
                .get_mut_map()
                .get_mut(&piece)
                .unwrap()
                .set(x_to, y_to);
        };
    }

    fn parse_pieces(fen: String) -> Result<(SideMap, SideMap)> {
        let mut w_pieces: SideMap = SideMap::new();
        let mut b_pieces: SideMap = SideMap::new();

        let mut rank: u32 = 7;
        let mut file: u32 = 0;

        for (index, c) in fen.as_str().chars().enumerate() {
            if c.is_ascii_digit() {
                if file + c.to_digit(10).unwrap() <= 8 {
                    file += c.to_digit(10).unwrap();
                } else {
                    return Err(anyhow!(
                        "Invalid character '{c}' at position {index} in FEN"
                    ));
                }
            } else if let Some(p) = PieceType::from_char(c) {
                let map: &mut SideMap = if c.is_ascii_uppercase() {
                    &mut w_pieces
                } else {
                    &mut b_pieces
                };

                map.toggle(p, file, rank);
                file += 1;
            } else if c == '/' {
            } else {
                return Err(anyhow!(
                    "Invalid character '{c}' at position {index} in FEN"
                ));
            }

            if file == 8 && rank > 0 {
                file = 0;
                rank -= 1;
            }
        }

        Ok((w_pieces, b_pieces))
    }

    fn parse_castling(fen: String) -> Result<(Castling, Castling)> {
        if fen.as_str() == "-" {
            return Ok((
                Castling {
                    king_side: false,
                    queen_side: false,
                },
                Castling {
                    king_side: false,
                    queen_side: false,
                },
            ));
        }

        let w_castling = Castling {
            king_side: fen.as_str().contains('K'),
            queen_side: fen.as_str().contains('Q'),
        };

        let b_castling = Castling {
            king_side: fen.as_str().contains('k'),
            queen_side: fen.as_str().contains('q'),
        };

        Ok((w_castling, b_castling))
    }

    pub fn from_fen(fen: String) -> Result<Position> {
        let fen: Vec<&str> = fen.as_str().split(' ').collect();

        let pieces = fen[0].to_string();
        let (w_pieces, b_pieces) = Position::parse_pieces(pieces)?;
        let w_pieces_all = w_pieces.combine();
        let b_pieces_all = b_pieces.combine();

        let side = match fen[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(anyhow!("Invalid side color in FEN")),
        };

        let castling = fen[2].to_string();
        let (w_castling, b_castling) = Position::parse_castling(castling)?;

        let en_passant = Square::from_str(fen[3]);

        let halfturn: usize = fen[4].parse()?;

        Ok(Position {
            side,
            halfturn,
            w_castling,
            b_castling,
            en_passant,
            w_pieces_all,
            w_pieces,
            b_pieces_all,
            b_pieces,
            history: vec![],
            legal_moves: vec![],
        })
    }

    /// Useful for displaying the position in a terminal.
    /// Lowecase letters refer to black pieces, uppercase refers to white.
    pub fn into_char_vec(&self) -> Vec<char> {
        let mut out = vec![' '; 64];

        for (ptype, board) in self.w_pieces.get_map() {
            let c = ptype.to_char().to_ascii_uppercase();

            for x in 0..8 {
                for y in 0..8 {
                    if board.get(x, y) == Some(true) {
                        out[y as usize * 8 + (7 - x as usize)] = c;
                    }
                }
            }
        }

        for (ptype, board) in self.b_pieces.get_map() {
            let c = ptype.to_char();

            for x in 0..8 {
                for y in 0..8 {
                    if board.get(x, y) == Some(true) {
                        out[y as usize * 8 + (7 - x as usize)] = c;
                    }
                }
            }
        }

        out
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub ptype: PieceType,
    pub change: StateChange,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StateChange {
    captured: Option<PieceType>,
}

#[cfg(test)]
mod tests {
    use crate::tui;

    use super::*;

    #[test]
    fn from_fen() {
        // Starting position
        assert!(Position::from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        )
        .is_ok());

        // Invalid number of pieces
        assert!(Position::from_fen(
            "rnbqkbn/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        )
        .is_err());

        // Invalid number of pieces
        assert!(Position::from_fen(
            "rnbqkbnr/pppppppp/7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        )
        .is_err());
    }

    #[test]
    fn change_and_reverse() {
        let mut pos = Position::from_fen(
            "rnbqkb1r/pppp1ppp/5n2/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq f3 0 3".to_string(),
        )
        .unwrap();

        let mv = Move {
            from: Square::from_str("e5").unwrap(),
            to: Square::from_str("f4").unwrap(),
            ptype: PieceType::Pawn,
            change: StateChange {
                captured: Some(PieceType::Pawn),
            },
        };
        tui::print_position(&pos);

        let pos_before = pos.clone();

        pos.change_board(mv);
        println!("exf4");
        tui::print_position(&pos);

        println!("------\nreversing move\n-----");

        pos.reverse_last_change();

        let pos_after = pos.clone();

        assert_eq!(pos_before, pos_after);

        tui::print_position(&pos);
    }
}
