use crate::{
    pieces::{PIECE_TYPE_COUNT, PieceTypes},
    side::Side,
};
pub type Bitboard = u64;

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Board {
    pub bb_pieces: [[Bitboard; PIECE_TYPE_COUNT]; Side::Both as usize],
    pub bb_side: [Bitboard; 2],

    pub side_to_move: Side,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            bb_pieces: [
                [0xff00, 0x42, 0x24, 0x81, 0x8, 0x10],
                [
                    0xff000000000000,
                    0x4200000000000000,
                    0x2400000000000000,
                    0x8100000000000000,
                    0x800000000000000,
                    0x800000000000000,
                ],
            ],
            bb_side: [0xffff, 0xffff000000000000],
            side_to_move: Side::White,
        }
    }
}

impl TryFrom<&str> for Board {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Board {
    pub fn get_pieces(&self, side: Side, piece: PieceTypes) -> Bitboard {
        self.bb_pieces[side as usize][piece as usize]
    }
}
