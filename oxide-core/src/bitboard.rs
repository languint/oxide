use crate::board::Bitboard;

// Taken from: https://rustic-chess.org/board_representation/bitboards.html
pub fn print_bitboard(bitboard: Bitboard) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let char = if bitboard & mask != 0 { '1' } else { '0' };
            print!("{char} ");
        }
        println!();
    }
}
