use super::{eval, square};
use super::{BitBoard, Castling, Color, GameState, PieceType, Position, SideMap, Square};
use anyhow::{anyhow, Result};

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
        } else if let Ok((p, c)) = PieceType::from_char(c) {
            let map: &mut SideMap = match c {
                Color::White => &mut w_pieces,
                Color::Black => &mut b_pieces,
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
    let (w_pieces, b_pieces) = parse_pieces(pieces)?;
    let w_pieces_all = w_pieces.combine();
    let b_pieces_all = b_pieces.combine();

    let side = match fen[1] {
        "w" => Color::White,
        "b" => Color::Black,
        _ => return Err(anyhow!("Invalid side color in FEN")),
    };

    let castling = fen[2].to_string();
    let (w_castling, b_castling) = parse_castling(castling)?;

    let en_passant = match fen[3] {
        "-" => None,
        _ => {
            let sqr = Square::from_str(fen[3]);
            if let Ok(s) = sqr {
                Some(s)
            } else {
                None
            }
        }
    };

    let halfturn: usize = fen[4].parse()?;

    //TODO: Check gamestate.

    let mut pos = Position {
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
        gamestate: GameState::Normal,
        w_attacks_all: BitBoard::new(),
        w_attacks: SideMap::new(),
        b_attacks_all: BitBoard::new(),
        b_attacks: SideMap::new(),
    };

    pos.update_attack_maps();

    Ok(pos)
}
