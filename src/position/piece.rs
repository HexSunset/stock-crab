#![allow(dead_code)]

#[derive(PartialEq, Eq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    pub fn from_char(c: char) -> Option<Self> {
        use PieceType as Ptype;

        let c = c.to_lowercase().next().unwrap();
        match c {
            'k' => Some(Ptype::King),
            'q' => Some(Ptype::Queen),
            'r' => Some(Ptype::Rook),
            'b' => Some(Ptype::Bishop),
            'n' => Some(Ptype::Knight),
            'p' => Some(Ptype::Pawn),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            PieceType::King => 'k',
            PieceType::Queen => 'q',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            PieceType::Pawn => 'p',
        }
    }
}

pub enum Color {
    Black,
    White,
}
