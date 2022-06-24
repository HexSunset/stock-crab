#![allow(dead_code)]

mod bitboard;
mod piece;
mod square;

use bitboard::BitBoard;
use piece::{Color, PieceType};
use square::Square;
use std::collections::HashMap;

use anyhow::{anyhow, Result};

pub struct Castling {
    king_side: bool,
    queen_side: bool,
}

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

pub struct Position {
    side: Color,
    halfturn: usize,

    w_castling: Castling,

    b_castling: Castling,

    en_passant: Option<Square>,

    w_pieces_all: BitBoard,
    w_pieces: SideMap,

    w_attacks_all: BitBoard,
    w_attacks: SideMap,

    b_pieces_all: BitBoard,
    b_pieces: SideMap,

    b_attacks_all: BitBoard,
    b_attacks: SideMap,
}

impl Position {
    fn parse_pieces(fen: String) -> Result<(SideMap, SideMap)> {
        let mut w_pieces: SideMap = SideMap::new();
        let mut b_pieces: SideMap = SideMap::new();

        let mut rank: u32 = 0;
        let mut file: u32 = 0;

        for c in fen.as_str().chars() {
            if c.is_ascii_digit() {
                if file + c.to_digit(10).unwrap() <= 8 {
                    file += c.to_digit(10).unwrap();
                } else {
                    return Err(anyhow!("Invalid character '{c}' in FEN"));
                }
            } else if let Some(p) = PieceType::from_char(c) {
                let map: &mut SideMap = if c.is_ascii_uppercase() {
                    &mut w_pieces
                } else {
                    &mut b_pieces
                };

                map.toggle(p, file, rank);
                file += 1;
            }

            if file == 8 {
                file = 0;
                rank += 1;
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

        let mut w_castling = Castling {
            king_side: fen.as_str().contains('K'),
            queen_side: fen.as_str().contains('Q'),
        };

        let mut b_castling = Castling {
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

        let out = Ok(Position {
            side,
            halfturn,
            w_castling,
            b_castling,
            en_passant,
            w_pieces_all,
            w_pieces,
            w_attacks_all,
            w_attacks,
            b_pieces_all,
            b_pieces,
            b_attacks_all,
            b_attacks,
        });

        unimplemented!();
    }
}
