use std::fs::File;
use std::io::Write;

const SAVE_FILENAME: &str = "config.ron";

#[derive(Serialize, Deserialize)]
pub struct ConfigSave {
    pub fps: u32,
    pub mouse_sensibility: f32,
}

pub struct Config {
    fps: u32,
    dt: f32,
    pub mouse_sensibility: f32,
}

impl Config {
    pub fn from_save(save: ConfigSave) -> Self {
        let mut config = Config {
            fps: 0,
            dt: 0.,
            mouse_sensibility: save.mouse_sensibility,
        };
        config.set_fps(save.fps);
        config
    }

    pub fn to_save(&self) -> ConfigSave {
        ConfigSave {
            fps: self.fps,
            mouse_sensibility: self.mouse_sensibility,
        }
    }

    #[inline]
    pub fn set_fps(&mut self, fps: u32) {
        self.fps = fps;
        self.dt = 1.0 / fps as f32;
    }

    #[inline]
    pub fn fps(&self) -> u32 {
        self.fps
    }

    #[inline]
    pub fn dt(&self) -> f32 {
        self.dt
    }

    pub fn load() -> Self {
        let file = File::open(SAVE_FILENAME).unwrap();
        Config::from_save(::ron::de::from_reader(file).unwrap())
    }

    pub fn save(&self) {
        let save = ::ron::ser::to_string(&self.to_save()).unwrap();
        let mut file = File::open(SAVE_FILENAME).unwrap();
        file.write_all(save.as_bytes()).unwrap();
    }
}
