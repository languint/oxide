#[repr(i8)]
pub enum PieceTypes {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const PIECE_TYPE_COUNT: usize = 6;