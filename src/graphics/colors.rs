pub fn colors() -> Vec<[u8; 4]> {
    vec![
        [0x00, 0x00, 0x00, 0xff],
        [0xef, 0xc5, 0x73, 0xff],
        [0x9c, 0x66, 0x3a, 0xff],
        [0xe5, 0x58, 0x5e, 0xff],
        [0xcb, 0x1d, 0x05, 0xff],
        [0x83, 0xd0, 0xe0, 0xff],
        [0x55, 0x8c, 0xcc, 0xff],
        [0xfc, 0xce, 0x79, 0xff],
        [0xff, 0xff, 0x44, 0xff],
        [0xad, 0xdf, 0x42, 0xff],
        [0x75, 0xce, 0x38, 0xff],
        [0xf6, 0xc6, 0xd2, 0xff],
        [0xc7, 0x5f, 0x7a, 0xff],
        [0xe2, 0xc5, 0xc9, 0xff],
        [0x97, 0x5d, 0xb2, 0xff],
    ]
}

#[allow(unused)]
pub mod color {
    pub const BLACK:       u16 =  0; //000000
    pub const PALE_BROWN:  u16 =  1; //efc573
    pub const BROWN:       u16 =  2; //9c663a
    pub const PALE_RED:    u16 =  3; //e5585e
    pub const RED:         u16 =  4; //cb1d05
    pub const PALE_BLUE:   u16 =  5; //83d0e0
    pub const BLUE:        u16 =  6; //558ccc
    pub const PALE_YELLOW: u16 =  7; //fcce79
    pub const YELLOW:      u16 =  8; //ffff44
    pub const PALE_GREEN:  u16 =  9; //addf42
    pub const GREEN:       u16 = 10; //75ce38
    pub const PALE_PINK:   u16 = 11; //f6c6d2
    pub const PINK:        u16 = 12; //c75f7a
    pub const PALE_PURPLE: u16 = 13; //e2c5c9
    pub const PURPLE:      u16 = 14; //975db2
}
