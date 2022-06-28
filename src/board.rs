#![allow(dead_code)]

mod bitboard;
mod eval;
mod movegen;
pub mod parse;
pub mod piece;
mod square;

use bitboard::BitBoard;
use movegen::Move;
use piece::{Color, PieceType};
use square::Square;
use std::collections::HashMap;

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

    pub fn get_board(&self, piece: PieceType) -> BitBoard {
        self.0.get(&piece).unwrap().clone()
    }

    pub fn get_mut_board(&mut self, piece: PieceType) -> &mut BitBoard {
        self.0.get_mut(&piece).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub gamestate: GameState,

    pub side: Color,
    pub halfturn: usize,

    pub w_castling: Castling,

    pub b_castling: Castling,

    pub en_passant: Option<Square>,

    pub w_pieces_all: BitBoard,
    pub w_pieces: SideMap,

    pub w_attacks_all: BitBoard,
    pub w_attacks: SideMap,

    pub b_pieces_all: BitBoard,
    pub b_pieces: SideMap,

    pub b_attacks_all: BitBoard,
    pub b_attacks: SideMap,

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

    fn update_attack_maps(&mut self) {
        let w_pieces = &self.w_pieces;
        let mut new_w_attacks = SideMap::new();

        for (piece, board) in new_w_attacks.get_mut_map() {
            *board = piece::get_piece_attack_map(
                *piece,
                w_pieces.get_map().get(piece).unwrap(),
                &self.w_pieces_all,
                &self.b_pieces_all,
            );
        }
        let new_w_attacks_all = new_w_attacks.combine();

        let b_pieces = &self.b_pieces;
        let mut new_b_attacks = SideMap::new();

        for (piece, board) in new_b_attacks.get_mut_map() {
            *board = piece::get_piece_attack_map(
                *piece,
                b_pieces.get_map().get(piece).unwrap(),
                &self.b_pieces_all,
                &self.w_pieces_all,
            );
        }
        let new_b_attacks_all = new_b_attacks.combine();

        self.w_attacks = new_w_attacks;
        self.w_attacks_all = new_w_attacks_all;
        self.b_attacks = new_b_attacks;
        self.b_attacks_all = new_b_attacks_all;
    }

    /// Useful for displaying the position in a terminal.
    /// Lowecase letters refer to black pieces, uppercase refers to white.
    pub fn into_char_vec(&self) -> Vec<char> {
        let mut out = vec![' '; 64];

        for (ptype, board) in self.w_pieces.get_map() {
            let c = ptype.to_char(Color::White);

            for x in 0..8 {
                for y in 0..8 {
                    if board.get(x, y) == Some(true) {
                        out[y as usize * 8 + (7 - x as usize)] = c;
                    }
                }
            }
        }

        for (ptype, board) in self.b_pieces.get_map() {
            let c = ptype.to_char(Color::Black);

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

#[derive(Clone, PartialEq, Debug)]
pub enum GameState {
    Normal,
    InCheck(Color),
    Draw,
    Won(Color),
}

#[cfg(test)]
mod tests {
    use crate::tui;

    use super::*;

    #[test]
    fn from_fen() {
        // Starting position
        assert!(parse::from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        )
        .is_ok());

        // Invalid number of pieces
        assert!(parse::from_fen(
            "rnbqkbn/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        )
        .is_err());

        // Invalid number of pieces
        assert!(parse::from_fen(
            "rnbqkbnr/pppppppp/7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        )
        .is_err());
    }

    #[test]
    fn change_and_reverse() {
        let mut pos = parse::from_fen(
            "rnbqkb1r/pppp1ppp/5n2/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq f3 0 3".to_string(),
        )
        .unwrap();

        let mv = Move {
            from: Square::from_str("e5").unwrap(),
            to: Square::from_str("f4").unwrap(),
            ptype: PieceType::Pawn,
            change: movegen::StateChange {
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
