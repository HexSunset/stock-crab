mod bitboard;
mod square;

use bitboard::BitBoard;
use square::Square;
use std::collections::HashMap;

pub struct Castling {
    king_side: bool,
    queen_side: bool,
}

pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub enum Color {
    Black,
    White,
}

pub struct Position {
    side: Color,
    halfturn: usize,

    w_castling: Castling,

    b_castling: Castling,

    en_passant: Option<Square>,

    w_pieces_all: BitBoard,
    w_pieces: HashMap<PieceType, BitBoard>,

    w_attacks_all: BitBoard,
    w_attacks: HashMap<PieceType, BitBoard>,

    b_pieces_all: BitBoard,
    b_pieces: HashMap<PieceType, BitBoard>,

    b_attacks_all: BitBoard,
    b_attacks: HashMap<PieceType, BitBoard>,
}

impl Position {
    //TODO: Proper error handling with anyhow.
    fn from_fen(fen: String) -> Option<Self> {
        todo!()
    }
}
