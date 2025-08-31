use crate::{pieces::{PieceTypes, PIECE_TYPE_COUNT}, side::Side};
pub type Bitboard = u64;

pub struct Board {
    pub bb_pieces: [[Bitboard; PIECE_TYPE_COUNT]; Side::Both as usize],
    pub bb_side: [Bitboard; 2],
}

impl Board {
    pub fn get_pieces(&self, side: Side, piece: PieceTypes) -> Bitboard {
        self.bb_pieces[side as usize][piece as usize]
    }
}