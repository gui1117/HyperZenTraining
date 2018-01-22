const MAX_DIVISION: usize = 20;

fn color_circle(x: f32) -> [f32; 3] {
    if x*6.0 < 1.0 {
        let t = x*6.0;
        [1.0, t, 0.0]
    } else if x*6.0 < 2.0 {
        let t = x*6.0-1.0;
        [1.0-t, 1.0, 0.0]
    } else if x*6.0 < 3.0 {
        let t = x*6.0-2.0;
        [0.0, 1.0, t]
    } else if x*6.0 < 4.0 {
        let t = x*6.0-3.0;
        [0.0, 1.0-t, 1.0]
    } else if x*6.0 < 5.0 {
        let t = x*6.0-4.0;
        [t, 0.0, 1.0]
    } else {
        let t = x*6.0-5.0;
        [1.0, 0.0, 1.0-t]
    }
}

pub fn colors() -> Vec<[f32; 4]> {
    let mut colors = vec![
        [0x00, 0x00, 0x00, 0xff],
        [0xff, 0xff, 0xff, 0xff],
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
        .collect::<Vec<_>>();

    assert!(colors.len() == Color::GenPaleBlack as usize);
    colors.append(&mut generate_colors(
        ::CONFIG.pale_gen_color_division,
        ::CONFIG.pale_gen_color_delta,
        ::CONFIG.pale_gen_color_black,
        ::CONFIG.pale_gen_color_white
    ));

    assert!(colors.len() == Color::GenBlack as usize);
    colors.append(&mut generate_colors(
        ::CONFIG.gen_color_division,
        ::CONFIG.gen_color_delta,
        ::CONFIG.gen_color_black,
        ::CONFIG.gen_color_white
    ));

    assert!(colors.len() - 1 == Color::Gen19 as usize);
    colors
}

fn generate_colors(division: usize, delta: f32, black: f32, white: f32) -> Vec<[f32; 4]> {
    let mut colors = vec![];
    assert!(black < white);
    assert!(division <= MAX_DIVISION);

    // Black
    colors.push([
        black,
        black,
        black,
        1.0
    ]);

    // White
    colors.push([
        white,
        white,
        white,
        1.0
    ]);

    for i in 0..division {
        let color = color_circle((i as f32 + delta) / division as f32);
        colors.push([
            color[0]*(white-black)+black,
            color[1]*(white-black)+black,
            color[2]*(white-black)+black,
            1.0
        ]);
    }

    for _ in 0..(MAX_DIVISION - division) {
        colors.push([
            0.0,
            0.0,
            0.0,
            1.0
        ]);
    }

    colors
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[repr(C)]
pub enum Color {
    Black,      //000000
    White,      //ffffff
    PaleBrown,  //efc573
    Brown,      //9c663a
    PaleRed,    //e5585e
    Red,        //cb1d05
    PaleBlue,   //83d0e0
    Blue,       //558ccc
    PaleYellow, //fcce79
    Yellow,     //ffff44
    PaleGreen,  //addf42
    Green,      //75ce38
    PalePink,   //f6c6d2
    Pink,       //c75f7a
    PalePurple, //e2c5c9
    Purple,     //975db2
    DarkBlue,   //416070
    GenPaleBlack,
    GenPaleWhite,
    GenPale0,
    GenPale1,
    GenPale2,
    GenPale3,
    GenPale4,
    GenPale5,
    GenPale6,
    GenPale7,
    GenPale8,
    GenPale9,
    GenPale10,
    GenPale11,
    GenPale12,
    GenPale13,
    GenPale14,
    GenPale15,
    GenPale16,
    GenPale17,
    GenPale18,
    GenPale19,
    GenBlack,
    GenWhite,
    Gen0,
    Gen1,
    Gen2,
    Gen3,
    Gen4,
    Gen5,
    Gen6,
    Gen7,
    Gen8,
    Gen9,
    Gen10,
    Gen11,
    Gen12,
    Gen13,
    Gen14,
    Gen15,
    Gen16,
    Gen17,
    Gen18,
    Gen19,
}
