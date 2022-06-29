use super::{piece::PieceType, Square};

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub ptype: PieceType,
    pub change: StateChange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateChange {
    pub captured: Option<PieceType>,
}
