use crate::side::Side;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingRight {
    King,
    Queen,
    Both,
}

pub type CastlingRights = [Option<CastlingRight>; 2];

pub fn try_from_str(s: &str) -> Result<CastlingRights, String> {
    if s == "-" {
        return Ok([None, None]);
    }

    let mut white_rights = (false, false);
    let mut black_rights = (false, false);

    for c in s.chars() {
        match c {
            'K' => white_rights.0 = true,
            'Q' => white_rights.1 = true,
            'k' => black_rights.0 = true,
            'q' => black_rights.1 = true,
            _ => return Err(format!("Invalid character in castling string: '{}'", c)),
        }
    }

    let mut castling_rights = [None, None];

    castling_rights[Side::White as usize] = match white_rights {
        (true, true) => Some(CastlingRight::Both),
        (true, false) => Some(CastlingRight::King),
        (false, true) => Some(CastlingRight::Queen),
        _ => None,
    };

    castling_rights[Side::Black as usize] = match black_rights {
        (true, true) => Some(CastlingRight::Both),
        (true, false) => Some(CastlingRight::King),
        (false, true) => Some(CastlingRight::Queen),
        _ => None,
    };

    Ok(castling_rights)
}