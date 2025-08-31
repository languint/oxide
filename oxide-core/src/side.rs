use std::ops;

#[repr(usize)]
pub enum Side {
    White,
    Black,
    Both
}

impl ops::Neg for Side {
    fn neg(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
            Self::Both => Self::Both
        }
    }
    type Output = Self;
}