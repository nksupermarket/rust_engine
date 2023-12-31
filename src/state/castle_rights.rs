use core::fmt;
use std::ops::{BitAnd, BitOr};

use crate::{
    bitboard::{self, BB},
    mv::castle::Castle,
    side::Side,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CastleRights(u8);

pub const NONE: CastleRights = CastleRights::new(0);
pub const KINGSIDE: CastleRights = CastleRights::new(0b1010);
pub const QUEENSIDE: CastleRights = CastleRights::new(0b0101);
pub const WHITE: CastleRights = CastleRights::new(0b1100);
pub const BLACK: CastleRights = CastleRights::new(0b0011);
pub const ALL: CastleRights = CastleRights::new(0b1111);
const COLOR_RIGHTS: [CastleRights; 2] = [WHITE, BLACK];
const CASTLE_RIGHTS: [CastleRights; 2] = [QUEENSIDE, KINGSIDE];

impl CastleRights {
    pub const fn new(u8: u8) -> CastleRights {
        CastleRights(u8)
    }

    pub fn to_u32(self) -> u32 {
        self.0 as u32
    }

    pub fn set(self, side: Side, castle: Castle) -> CastleRights {
        CastleRights(
            self.0 | (COLOR_RIGHTS[side.to_usize()].0 & CASTLE_RIGHTS[castle.to_usize()].0),
        )
    }

    pub fn can(&self, side: Side, castle: Castle) -> bool {
        let side_rights = if side == Side::White { WHITE } else { BLACK };
        let castle_rights = if castle == Castle::Kingside {
            KINGSIDE
        } else {
            QUEENSIDE
        };
        let rights = castle_rights.0 & side_rights.0;

        (self.0 & rights) != 0
    }

    pub fn remove_rights(self, side: Side, castle: Castle) -> CastleRights {
        CastleRights(
            self.0 & !(COLOR_RIGHTS[side.to_usize()].0 & CASTLE_RIGHTS[castle.to_usize()].0),
        )
    }

    pub fn remove_rights_for_color(self, side: Side) -> CastleRights {
        CastleRights(self.0 & !(COLOR_RIGHTS[side.to_usize()].0))
    }

    pub fn iter(self) -> CastleRightsIterator {
        CastleRightsIterator(BB(self.0 as u64))
    }
}

pub struct CastleRightsIterator(BB);
impl Iterator for CastleRightsIterator {
    type Item = usize;

    // iterates a bitboard from low to high
    fn next(&mut self) -> Option<usize> {
        if self.0 == bitboard::EMPTY {
            return None;
        }

        let bb = BB(self.0 .0);

        let lsb = bb.lsb();
        self.0 ^= lsb;

        Some(lsb.to_usize())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn set1() {
        let cr = KINGSIDE;
        let expected = "Kk";

        assert_eq!(cr.to_string(), expected);
    }

    #[test]
    pub fn set2() {
        let cr = NONE.set(Side::White, Castle::Kingside);
        let expected = "K";

        assert_eq!(cr.to_string(), expected);
    }

    #[test]
    pub fn set3() {
        let cr = NONE.set(Side::White, Castle::Kingside);
        let cr = cr.set(Side::White, Castle::Queenside);
        let expected = "KQ";

        assert_eq!(cr.to_string(), expected);
    }

    #[test]
    pub fn can1() {
        let cr = KINGSIDE;

        assert_eq!(cr.can(Side::White, Castle::Kingside), true);
    }

    #[test]
    pub fn can2() {
        let cr = KINGSIDE;

        assert_eq!(cr.can(Side::White, Castle::Queenside), false);
    }
    #[test]
    pub fn can3() {
        let cr = WHITE;

        assert_eq!(cr.can(Side::White, Castle::Queenside), true);
    }
    #[test]
    pub fn can4() {
        let cr = BLACK;

        assert_eq!(cr.can(Side::Black, Castle::Kingside), true);
    }
    #[test]
    pub fn can5() {
        let cr = BLACK;

        assert_eq!(cr.can(Side::White, Castle::Kingside), false);
    }
}

impl BitOr for CastleRights {
    type Output = CastleRights;

    fn bitor(self, other: CastleRights) -> CastleRights {
        CastleRights(self.0 | other.0)
    }
}
impl BitAnd for CastleRights {
    type Output = CastleRights;

    fn bitand(self, other: CastleRights) -> CastleRights {
        CastleRights(self.0 & other.0)
    }
}

impl fmt::Display for CastleRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        if self.0 == 0 {
            str += "-"
        } else {
            if (self.0 & (KINGSIDE & WHITE).0) != 0 {
                str += "K"
            }
            if (self.0 & (QUEENSIDE & WHITE).0) != 0 {
                str += "Q"
            }
            if (self.0 & (KINGSIDE & BLACK).0) != 0 {
                str += "k"
            }
            if (self.0 & (QUEENSIDE & BLACK).0) != 0 {
                str += "q"
            }
        }
        write!(f, "{}", str)
    }
}

#[cfg(test)]
pub mod test_display {
    use super::*;

    #[test]
    pub fn none() {
        let fmt_str = NONE.to_string();
        let expected = "-";

        assert_eq!(fmt_str, expected);
    }
    #[test]
    pub fn kingside() {
        let fmt_str = KINGSIDE.to_string();
        let expected = "Kk";

        assert_eq!(fmt_str, expected);
    }
    #[test]
    pub fn queenside() {
        let fmt_str = QUEENSIDE.to_string();
        let expected = "Qq";

        assert_eq!(fmt_str, expected);
    }
    #[test]
    pub fn white() {
        let fmt_str = WHITE.to_string();
        let expected = "KQ";

        assert_eq!(fmt_str, expected);
    }
    #[test]
    pub fn black() {
        let fmt_str = BLACK.to_string();
        let expected = "kq";

        assert_eq!(fmt_str, expected);
    }
}
