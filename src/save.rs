extern crate app_dirs;
extern crate mut_static;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde;

use mut_static::MutStatic;

use std::fs::File;

const SAVE_FILENAME: &str = "save.yml";

#[derive(Serialize, Deserialize)]
pub struct Save {
    physical_device: Option<String>,
}

macro_rules! write_save {
    ($($tt:tt)*) => {
        {
            let mut save = ::SAVE.write().unwrap();
            save.$($tt)*;
            save.write_to_file();
        }
    }
}

impl Save {
    pub fn init() -> Self {
        let app_info = app_dirs::AppInfo {
            name: env!("CARGO_PKG_NAME"),
            author: env!("CARGO_PKG_AUTHORS"),
        };
        let mut save_path = app_dirs::app_root(app_dirs::AppDataType::UserConfig, &app_info).expect("failed to get/set user config dir");
        save_path.push(SAVE_FILENAME);
        if !save_path.is_file() {
            let mut file = File::create(save_path.clone()).expect("failed to create save file");
            let save = Save {
                physical_device: None,
            };
            serde_yaml::to_writer(file, &save).unwrap();
        }
        let file = File::open(save_path).unwrap();
        serde_yaml::from_reader(file).unwrap()
    }
    pub fn write_to_file(&self) {
        let mut save_path = app_dirs::app_root(app_dirs::AppDataType::UserConfig, &app_info).expect("failed to get/set user config dir");
        save_path.push(SAVE_FILENAME);
        let file = File::open(save_path).unwrap();
        serde_yaml::to_writer(file, self).expect("failed to write save file")
    }
}

lazy_static! {
    pub static ref SAVE: MutStatic<Save> = {
        MutStatic::from(Save::init())
    };
}
