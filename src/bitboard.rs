pub mod squares_between;

use crate::util::grid_to_string;
use std::{
    fmt,
    ops::{
        Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Neg, Not, Shl,
        Shr, Sub,
    },
};

use crate::square::Square;

pub const BOARD_LENGTH: usize = 64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BB(pub u64);

impl BB {
    pub fn new(sq: Square) -> BB {
        BB(1u64 << sq.0)
    }

    pub fn to_usize(self) -> usize {
        self.0.trailing_zeros() as usize
    }

    pub fn lsb(self) -> BB {
        BB(self.0 & (0u64.wrapping_sub(self.0)))
    }

    pub fn bitscan(self) -> Square {
        Square::new(self.0.trailing_zeros() as usize)
    }

    pub fn is_set(self, sq: Square) -> bool {
        (self.0 >> sq.0) & 1 != 0
    }

    pub fn from_arr(sq_arr: &[Square]) -> BB {
        let mut bb = EMPTY;
        for sq in sq_arr {
            bb |= BB::new(*sq);
        }
        bb
    }

    pub fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    pub fn reverse(self) -> BB {
        BB(self.0.reverse_bits())
    }

    pub fn empty(self) -> bool {
        self.0 == 0u64
    }

    pub fn not_empty(self) -> bool {
        self.0 != 0u64
    }

    pub fn iter(self) -> BBIterator {
        BBIterator(self)
    }
}

impl Shr<usize> for BB {
    type Output = BB;

    fn shr(self, amount: usize) -> BB {
        BB(self.0 >> amount)
    }
}

impl Shl<usize> for BB {
    type Output = BB;

    fn shl(self, amount: usize) -> BB {
        BB(self.0 << amount)
    }
}

impl Not for BB {
    type Output = BB;

    fn not(self) -> BB {
        BB(!self.0)
    }
}

impl BitOr for BB {
    type Output = BB;

    fn bitor(self, other: BB) -> BB {
        BB(self.0 | other.0)
    }
}

impl BitOrAssign for BB {
    fn bitor_assign(&mut self, other: BB) {
        self.0 |= other.0
    }
}

impl BitXor for BB {
    type Output = BB;

    fn bitxor(self, other: BB) -> BB {
        BB(self.0 ^ other.0)
    }
}

impl BitXorAssign for BB {
    fn bitxor_assign(&mut self, other: BB) {
        self.0 ^= other.0
    }
}

impl BitAnd for BB {
    type Output = BB;

    fn bitand(self, other: BB) -> BB {
        BB(self.0 & other.0)
    }
}

impl BitAndAssign for BB {
    fn bitand_assign(&mut self, other: BB) {
        self.0 &= other.0
    }
}

impl Sub for BB {
    type Output = BB;

    fn sub(self, other: BB) -> BB {
        BB(self.0.wrapping_sub(other.0))
    }
}

impl Add for BB {
    type Output = BB;

    fn add(self, other: BB) -> BB {
        BB(self.0.wrapping_add(other.0))
    }
}

impl Mul for BB {
    type Output = BB;

    fn mul(self, other: BB) -> BB {
        BB(self.0.wrapping_mul(other.0))
    }
}

impl Neg for BB {
    type Output = BB;

    fn neg(self) -> BB {
        BB(self.0.wrapping_neg())
    }
}

pub const EMPTY: BB = BB(0);
pub const END_ROWS: BB = BB(ROW_1.0 | ROW_8.0);
pub const FILE_A: BB = BB(0x0101010101010101u64);
pub const FILE_B: BB = BB(FILE_A.0 << 1);
pub const FILE_G: BB = BB(FILE_A.0 << 6);
pub const FILE_H: BB = BB(FILE_A.0 << 7);
pub const NOT_FILE_A: BB = BB(!FILE_A.0);
pub const NOT_FILE_H: BB = BB(!FILE_H.0);
pub const ROW_1: BB = BB(0xFFu64);
pub const ROW_2: BB = BB(ROW_1.0 << (8));
pub const ROW_4: BB = BB(ROW_1.0 << (3 * 8));
pub const ROW_5: BB = BB(ROW_1.0 << (4 * 8));
pub const ROW_7: BB = BB(ROW_1.0 << (6 * 8));
pub const ROW_8: BB = BB(ROW_1.0 << (7 * 8));
pub const EDGES: BB = BB(FILE_A.0 | FILE_H.0 | ROW_1.0 | ROW_8.0);
pub const INNER_CENTER: BB = BB(103481868288);
pub const OUTER_CENTER: BB = BB(66125924401152);

impl fmt::Display for BB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            grid_to_string(|sq: Square| -> char {
                if self.is_set(sq) {
                    '#'
                } else {
                    '.'
                }
            })
        )
    }
}

pub struct BBIterator(BB);

impl Iterator for BBIterator {
    type Item = Square;

    // iterates a bitboard from low to high
    fn next(&mut self) -> Option<Square> {
        if (self.0).0 == EMPTY.0 {
            return None;
        }

        let sq = self.0.bitscan();
        let lsb = self.0.lsb();
        self.0 ^= lsb;
        Some(sq)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test() {
        let val = 0u64.wrapping_sub(1u64);
        println!("{}", val)
    }

    #[test]
    fn consts_1() {
        let expected = unindent::unindent(
            "
              ABCDEFGH
            8|#.......|8
            7|#.......|7
            6|#.......|6
            5|#.......|5
            4|#.......|4
            3|#.......|3
            2|#.......|2
            1|#.......|1
              ABCDEFGH
            ",
        );
        assert_eq!(FILE_A.to_string(), expected);
    }

    #[test]
    fn consts_2() {
        let expected = unindent::unindent(
            "
              ABCDEFGH
            8|.......#|8
            7|.......#|7
            6|.......#|6
            5|.......#|5
            4|.......#|4
            3|.......#|3
            2|.......#|2
            1|.......#|1
              ABCDEFGH
            ",
        );
        assert_eq!(FILE_H.to_string(), expected);
    }

    #[test]
    fn consts_4() {
        let expected = unindent::unindent(
            "
              ABCDEFGH
            8|########|8
            7|........|7
            6|........|6
            5|........|5
            4|........|4
            3|........|3
            2|........|2
            1|########|1
              ABCDEFGH
            ",
        );
        assert_eq!(END_ROWS.to_string(), expected);
    }

    #[test]
    fn consts_5() {
        let expected = unindent::unindent(
            "
              ABCDEFGH
            8|........|8
            7|........|7
            6|........|6
            5|...##...|5
            4|...##...|4
            3|........|3
            2|........|2
            1|........|1
              ABCDEFGH
            ",
        );
        assert_eq!(INNER_CENTER.to_string(), expected);
    }

    #[test]
    fn consts_6() {
        let expected = unindent::unindent(
            "
              ABCDEFGH
            8|........|8
            7|........|7
            6|..####..|6
            5|..#..#..|5
            4|..#..#..|4
            3|..####..|3
            2|........|2
            1|........|1
              ABCDEFGH
            ",
        );
        println!(
            "expected: {}\nreceived: {}",
            expected,
            OUTER_CENTER.to_string()
        );
        assert_eq!(OUTER_CENTER.to_string(), expected);
    }
}

pub static BISHOP_RAYS: [BB; BOARD_LENGTH] = [
    BB(9241421688590303744),
    BB(36099303471056128),
    BB(141012904249856),
    BB(550848566272),
    BB(6480472064),
    BB(1108177604608),
    BB(283691315142656),
    BB(72624976668147712),
    BB(4620710844295151618),
    BB(9241421688590368773),
    BB(36099303487963146),
    BB(141017232965652),
    BB(1659000848424),
    BB(283693466779728),
    BB(72624976676520096),
    BB(145249953336262720),
    BB(2310355422147510788),
    BB(4620710844311799048),
    BB(9241421692918565393),
    BB(36100411639206946),
    BB(424704217196612),
    BB(72625527495610504),
    BB(145249955479592976),
    BB(290499906664153120),
    BB(1155177711057110024),
    BB(2310355426409252880),
    BB(4620711952330133792),
    BB(9241705379636978241),
    BB(108724279602332802),
    BB(145390965166737412),
    BB(290500455356698632),
    BB(580999811184992272),
    BB(577588851267340304),
    BB(1155178802063085600),
    BB(2310639079102947392),
    BB(4693335752243822976),
    BB(9386671504487645697),
    BB(326598935265674242),
    BB(581140276476643332),
    BB(1161999073681608712),
    BB(288793334762704928),
    BB(577868148797087808),
    BB(1227793891648880768),
    BB(2455587783297826816),
    BB(4911175566595588352),
    BB(9822351133174399489),
    BB(1197958188344280066),
    BB(2323857683139004420),
    BB(144117404414255168),
    BB(360293502378066048),
    BB(720587009051099136),
    BB(1441174018118909952),
    BB(2882348036221108224),
    BB(5764696068147249408),
    BB(11529391036782871041),
    BB(4611756524879479810),
    BB(567382630219904),
    BB(1416240237150208),
    BB(2833579985862656),
    BB(5667164249915392),
    BB(11334324221640704),
    BB(22667548931719168),
    BB(45053622886727936),
    BB(18049651735527937),
];

pub static ROOK_RAYS: [BB; BOARD_LENGTH] = [
    BB(72340172838076926),
    BB(144680345676153597),
    BB(289360691352306939),
    BB(578721382704613623),
    BB(1157442765409226991),
    BB(2314885530818453727),
    BB(4629771061636907199),
    BB(9259542123273814143),
    BB(72340172838141441),
    BB(144680345676217602),
    BB(289360691352369924),
    BB(578721382704674568),
    BB(1157442765409283856),
    BB(2314885530818502432),
    BB(4629771061636939584),
    BB(9259542123273813888),
    BB(72340172854657281),
    BB(144680345692602882),
    BB(289360691368494084),
    BB(578721382720276488),
    BB(1157442765423841296),
    BB(2314885530830970912),
    BB(4629771061645230144),
    BB(9259542123273748608),
    BB(72340177082712321),
    BB(144680349887234562),
    BB(289360695496279044),
    BB(578721386714368008),
    BB(1157442769150545936),
    BB(2314885534022901792),
    BB(4629771063767613504),
    BB(9259542123257036928),
    BB(72341259464802561),
    BB(144681423712944642),
    BB(289361752209228804),
    BB(578722409201797128),
    BB(1157443723186933776),
    BB(2314886351157207072),
    BB(4629771607097753664),
    BB(9259542118978846848),
    BB(72618349279904001),
    BB(144956323094725122),
    BB(289632270724367364),
    BB(578984165983651848),
    BB(1157687956502220816),
    BB(2315095537539358752),
    BB(4629910699613634624),
    BB(9259541023762186368),
    BB(143553341945872641),
    BB(215330564830528002),
    BB(358885010599838724),
    BB(645993902138460168),
    BB(1220211685215703056),
    BB(2368647251370188832),
    BB(4665518383679160384),
    BB(9259260648297103488),
    BB(18302911464433844481),
    BB(18231136449196065282),
    BB(18087586418720506884),
    BB(17800486357769390088),
    BB(17226286235867156496),
    BB(16077885992062689312),
    BB(13781085504453754944),
    BB(9187484529235886208),
];

pub static KNIGHT_JUMPS: [BB; BOARD_LENGTH] = [
    BB(0x0000000000020400u64),
    BB(0x0000000000050800u64),
    BB(0x00000000000A1100u64),
    BB(0x0000000000142200u64),
    BB(0x0000000000284400u64),
    BB(0x0000000000508800u64),
    BB(0x0000000000A01000u64),
    BB(0x0000000000402000u64),
    BB(0x0000000002040004u64),
    BB(0x0000000005080008u64),
    BB(0x000000000A110011u64),
    BB(0x0000000014220022u64),
    BB(0x0000000028440044u64),
    BB(0x0000000050880088u64),
    BB(0x00000000A0100010u64),
    BB(0x0000000040200020u64),
    BB(0x0000000204000402u64),
    BB(0x0000000508000805u64),
    BB(0x0000000A1100110Au64),
    BB(0x0000001422002214u64),
    BB(0x0000002844004428u64),
    BB(0x0000005088008850u64),
    BB(0x000000A0100010A0u64),
    BB(0x0000004020002040u64),
    BB(0x0000020400040200u64),
    BB(0x0000050800080500u64),
    BB(0x00000A1100110A00u64),
    BB(0x0000142200221400u64),
    BB(0x0000284400442800u64),
    BB(0x0000508800885000u64),
    BB(0x0000A0100010A000u64),
    BB(0x0000402000204000u64),
    BB(0x0002040004020000u64),
    BB(0x0005080008050000u64),
    BB(0x000A1100110A0000u64),
    BB(0x0014220022140000u64),
    BB(0x0028440044280000u64),
    BB(0x0050880088500000u64),
    BB(0x00A0100010A00000u64),
    BB(0x0040200020400000u64),
    BB(0x0204000402000000u64),
    BB(0x0508000805000000u64),
    BB(0x0A1100110A000000u64),
    BB(0x1422002214000000u64),
    BB(0x2844004428000000u64),
    BB(0x5088008850000000u64),
    BB(0xA0100010A0000000u64),
    BB(0x4020002040000000u64),
    BB(0x0400040200000000u64),
    BB(0x0800080500000000u64),
    BB(0x1100110A00000000u64),
    BB(0x2200221400000000u64),
    BB(0x4400442800000000u64),
    BB(0x8800885000000000u64),
    BB(0x100010A000000000u64),
    BB(0x2000204000000000u64),
    BB(0x0004020000000000u64),
    BB(0x0008050000000000u64),
    BB(0x00110A0000000000u64),
    BB(0x0022140000000000u64),
    BB(0x0044280000000000u64),
    BB(0x0088500000000000u64),
    BB(0x0010A00000000000u64),
    BB(0x0020400000000000u64),
];

pub static KING_MOVES: [BB; BOARD_LENGTH] = [
    BB(0x0000000000000302u64),
    BB(0x0000000000000705u64),
    BB(0x0000000000000E0Au64),
    BB(0x0000000000001C14u64),
    BB(0x0000000000003828u64),
    BB(0x0000000000007050u64),
    BB(0x000000000000E0A0u64),
    BB(0x000000000000C040u64),
    BB(0x0000000000030203u64),
    BB(0x0000000000070507u64),
    BB(0x00000000000E0A0Eu64),
    BB(0x00000000001C141Cu64),
    BB(0x0000000000382838u64),
    BB(0x0000000000705070u64),
    BB(0x0000000000E0A0E0u64),
    BB(0x0000000000C040C0u64),
    BB(0x0000000003020300u64),
    BB(0x0000000007050700u64),
    BB(0x000000000E0A0E00u64),
    BB(0x000000001C141C00u64),
    BB(0x0000000038283800u64),
    BB(0x0000000070507000u64),
    BB(0x00000000E0A0E000u64),
    BB(0x00000000C040C000u64),
    BB(0x0000000302030000u64),
    BB(0x0000000705070000u64),
    BB(0x0000000E0A0E0000u64),
    BB(0x0000001C141C0000u64),
    BB(0x0000003828380000u64),
    BB(0x0000007050700000u64),
    BB(0x000000E0A0E00000u64),
    BB(0x000000C040C00000u64),
    BB(0x0000030203000000u64),
    BB(0x0000070507000000u64),
    BB(0x00000E0A0E000000u64),
    BB(0x00001C141C000000u64),
    BB(0x0000382838000000u64),
    BB(0x0000705070000000u64),
    BB(0x0000E0A0E0000000u64),
    BB(0x0000C040C0000000u64),
    BB(0x0003020300000000u64),
    BB(0x0007050700000000u64),
    BB(0x000E0A0E00000000u64),
    BB(0x001C141C00000000u64),
    BB(0x0038283800000000u64),
    BB(0x0070507000000000u64),
    BB(0x00E0A0E000000000u64),
    BB(0x00C040C000000000u64),
    BB(0x0302030000000000u64),
    BB(0x0705070000000000u64),
    BB(0x0E0A0E0000000000u64),
    BB(0x1C141C0000000000u64),
    BB(0x3828380000000000u64),
    BB(0x7050700000000000u64),
    BB(0xE0A0E00000000000u64),
    BB(0xC040C00000000000u64),
    BB(0x0203000000000000u64),
    BB(0x0507000000000000u64),
    BB(0x0A0E000000000000u64),
    BB(0x141C000000000000u64),
    BB(0x2838000000000000u64),
    BB(0x5070000000000000u64),
    BB(0xA0E0000000000000u64),
    BB(0x40C0000000000000u64),
];

const W_PAWN_PUSHES: [BB; BOARD_LENGTH] = [
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(65536),
    BB(131072),
    BB(262144),
    BB(524288),
    BB(1048576),
    BB(2097152),
    BB(4194304),
    BB(8388608),
    BB(16777216),
    BB(33554432),
    BB(67108864),
    BB(134217728),
    BB(268435456),
    BB(536870912),
    BB(1073741824),
    BB(2147483648),
    BB(4294967296),
    BB(8589934592),
    BB(17179869184),
    BB(34359738368),
    BB(68719476736),
    BB(137438953472),
    BB(274877906944),
    BB(549755813888),
    BB(1099511627776),
    BB(2199023255552),
    BB(4398046511104),
    BB(8796093022208),
    BB(17592186044416),
    BB(35184372088832),
    BB(70368744177664),
    BB(140737488355328),
    BB(281474976710656),
    BB(562949953421312),
    BB(1125899906842624),
    BB(2251799813685248),
    BB(4503599627370496),
    BB(9007199254740992),
    BB(18014398509481984),
    BB(36028797018963968),
    BB(72057594037927936),
    BB(144115188075855872),
    BB(288230376151711744),
    BB(576460752303423488),
    BB(1152921504606846976),
    BB(2305843009213693952),
    BB(4611686018427387904),
    BB(9223372036854775808),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
];
const B_PAWN_PUSHES: [BB; BOARD_LENGTH] = [
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(1),
    BB(2),
    BB(4),
    BB(8),
    BB(16),
    BB(32),
    BB(64),
    BB(128),
    BB(256),
    BB(512),
    BB(1024),
    BB(2048),
    BB(4096),
    BB(8192),
    BB(16384),
    BB(32768),
    BB(65536),
    BB(131072),
    BB(262144),
    BB(524288),
    BB(1048576),
    BB(2097152),
    BB(4194304),
    BB(8388608),
    BB(16777216),
    BB(33554432),
    BB(67108864),
    BB(134217728),
    BB(268435456),
    BB(536870912),
    BB(1073741824),
    BB(2147483648),
    BB(4294967296),
    BB(8589934592),
    BB(17179869184),
    BB(34359738368),
    BB(68719476736),
    BB(137438953472),
    BB(274877906944),
    BB(549755813888),
    BB(1099511627776),
    BB(2199023255552),
    BB(4398046511104),
    BB(8796093022208),
    BB(17592186044416),
    BB(35184372088832),
    BB(70368744177664),
    BB(140737488355328),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
];

pub static PAWN_PUSHES: [[BB; BOARD_LENGTH]; 2] = [W_PAWN_PUSHES, B_PAWN_PUSHES];

const W_PAWN_CAPTURES: [BB; BOARD_LENGTH] = [
    BB(512),
    BB(1280),
    BB(2560),
    BB(5120),
    BB(10240),
    BB(20480),
    BB(40960),
    BB(16384),
    BB(131072),
    BB(327680),
    BB(655360),
    BB(1310720),
    BB(2621440),
    BB(5242880),
    BB(10485760),
    BB(4194304),
    BB(33554432),
    BB(83886080),
    BB(167772160),
    BB(335544320),
    BB(671088640),
    BB(1342177280),
    BB(2684354560),
    BB(1073741824),
    BB(8589934592),
    BB(21474836480),
    BB(42949672960),
    BB(85899345920),
    BB(171798691840),
    BB(343597383680),
    BB(687194767360),
    BB(274877906944),
    BB(2199023255552),
    BB(5497558138880),
    BB(10995116277760),
    BB(21990232555520),
    BB(43980465111040),
    BB(87960930222080),
    BB(175921860444160),
    BB(70368744177664),
    BB(562949953421312),
    BB(1407374883553280),
    BB(2814749767106560),
    BB(5629499534213120),
    BB(11258999068426240),
    BB(22517998136852480),
    BB(45035996273704960),
    BB(18014398509481984),
    BB(144115188075855872),
    BB(360287970189639680),
    BB(720575940379279360),
    BB(1441151880758558720),
    BB(2882303761517117440),
    BB(5764607523034234880),
    BB(11529215046068469760),
    BB(4611686018427387904),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
];

const B_PAWN_CAPTURES: [BB; BOARD_LENGTH] = [
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(0),
    BB(2),
    BB(5),
    BB(10),
    BB(20),
    BB(40),
    BB(80),
    BB(160),
    BB(64),
    BB(512),
    BB(1280),
    BB(2560),
    BB(5120),
    BB(10240),
    BB(20480),
    BB(40960),
    BB(16384),
    BB(131072),
    BB(327680),
    BB(655360),
    BB(1310720),
    BB(2621440),
    BB(5242880),
    BB(10485760),
    BB(4194304),
    BB(33554432),
    BB(83886080),
    BB(167772160),
    BB(335544320),
    BB(671088640),
    BB(1342177280),
    BB(2684354560),
    BB(1073741824),
    BB(8589934592),
    BB(21474836480),
    BB(42949672960),
    BB(85899345920),
    BB(171798691840),
    BB(343597383680),
    BB(687194767360),
    BB(274877906944),
    BB(2199023255552),
    BB(5497558138880),
    BB(10995116277760),
    BB(21990232555520),
    BB(43980465111040),
    BB(87960930222080),
    BB(175921860444160),
    BB(70368744177664),
    BB(562949953421312),
    BB(1407374883553280),
    BB(2814749767106560),
    BB(5629499534213120),
    BB(11258999068426240),
    BB(22517998136852480),
    BB(45035996273704960),
    BB(18014398509481984),
];

pub static PAWN_CAPTURES: [[BB; BOARD_LENGTH]; 2] = [W_PAWN_CAPTURES, B_PAWN_CAPTURES];
