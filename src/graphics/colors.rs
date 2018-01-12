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

    assert!(colors.len() == Color::Wall0 as usize);
    assert!(::CONFIG.wall_color_division <= 30);
    for i in 0..::CONFIG.wall_color_division {
        let color = color_circle(i as f32 / ::CONFIG.wall_color_division as f32);
        let blank = ::CONFIG.wall_color_blank;
        colors.push([
            color[0]*(1.0-blank)+blank,
            color[1]*(1.0-blank)+blank,
            color[2]*(1.0-blank)+blank,
            1.0
        ]);
    }

    colors
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[repr(C)]
pub enum Color {
    Black,      //000000
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
    Wall0,
    Wall1,
    Wall2,
    Wall3,
    Wall4,
    Wall5,
    Wall6,
    Wall7,
    Wall8,
    Wall9,
    Wall10,
    Wall11,
    Wall12,
    Wall13,
    Wall14,
    Wall15,
    Wall16,
    Wall17,
    Wall18,
    Wall19,
    Wall20,
    Wall21,
    Wall22,
    Wall23,
    Wall24,
    Wall25,
    Wall26,
    Wall27,
    Wall28,
    Wall29,
}
