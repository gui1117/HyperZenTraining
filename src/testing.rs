use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct Testing {
    pub a: usize,
    pub b: f32,
}

lazy_static! {
    pub static ref TS: Testing = {
        let file = File::open("testing.ron").unwrap();
        ::ron::de::from_reader(file).unwrap()
    };
}
