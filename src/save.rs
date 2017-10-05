extern crate app_dirs;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde;

use std::fs::File;

const SAVE_FILENAME: &str = concat!("config_", env!("CARGO_PKG_VERSION"), ".ron");

#[derive(Serialize, Deserialize)]
pub struct ConfigSave {
    pub fps: u32,
    pub mouse_sensibility: f32,
}

impl Default for ConfigSave {
    fn default() -> Self {
        let fps = 60;
        ConfigSave {
            fps: 60,
            mouse_sensibility: 0.0001,
        }
    }
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
}

// macro_rules! write_save {
//     ($($tt:tt)*) => {
//         {
//             let mut save = ::SAVE.write().unwrap();
//             save.$($tt)*;
//             save.write_to_file();
//         }
//     }
// }

impl Config {
    pub fn load() -> Self {
        let app_info = app_dirs::AppInfo {
            name: env!("CARGO_PKG_NAME"),
            author: env!("CARGO_PKG_AUTHORS"),
        };
        let mut save_path = app_dirs::app_root(app_dirs::AppDataType::UserConfig, &app_info).expect("failed to get/set user config dir");
        save_path.push(SAVE_FILENAME);
        if !save_path.is_file() {
            let mut file = File::create(save_path.clone()).expect("failed to create save file");

            serde_yaml::to_writer(file, &save).unwrap();
        }
        let file = File::open(save_path).unwrap();
        serde_yaml::from_reader(file).unwrap()
    }

    pub fn save(&self) {
        let mut save_path = app_dirs::app_root(app_dirs::AppDataType::UserConfig, &app_info).expect("failed to get/set user config dir");
        save_path.push(SAVE_FILENAME);
        let file = File::open(save_path).unwrap();
        serde_yaml::to_writer(file, self).expect("failed to write save file")
    }
}
