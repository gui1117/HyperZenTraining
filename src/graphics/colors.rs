pub fn colors() -> Vec<[f32; 4]> {
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
        [0x41, 0x60, 0x70, 0xff],
    ].into_iter()
        .map(|color| {
            let mut out_color = [0f32; 4];
            for i in 0..4 {
                out_color[i] = color[i] as f32 / 255.0;
            }
            out_color
        })
        .collect()
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Color {
    Black,        //000000
    PaleBrown,   //efc573
    Brown,        //9c663a
    PaleRed,     //e5585e
    Red,          //cb1d05
    PaleBlue,    //83d0e0
    Blue,         //558ccc
    PaleYellow,  //fcce79
    Yellow,       //ffff44
    PaleGreen,   //addf42
    Green,        //75ce38
    PalePink,    //f6c6d2
    Pink,         //c75f7a
    PalePurple,  //e2c5c9
    Purple,       //975db2
    DarkBlue,
}
