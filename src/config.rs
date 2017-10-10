use std::fs::File;
use std::io::Write;

const SAVE_FILENAME: &str = "config.ron";

macro_rules! config {
    (
        $(saved $saved_field:ident: $saved_type:ty,)*
        $(built $built_field:ident: $built_type:ty: $built_default:expr,)*
    ) => {
        #[derive(Serialize, Deserialize, Clone)]
        struct ConfigSave {
            $($saved_field: $saved_type,)*
        }

        pub struct Config {
            $($saved_field: $saved_type,)*
            $($built_field: $built_type,)*
        }

        impl Config {
            $(#[inline]
            pub fn $saved_field(&self) -> $saved_type{
                self.$saved_field
            })*
            $(#[inline]
            pub fn $built_field(&self) -> $built_type {
                self.$built_field
            })*

            fn from_save_default(save: ConfigSave) -> Self {
                Config {
                    $($saved_field: save.$saved_field,)*
                    $($built_field: $built_default,)*
                }
            }

            fn to_save(&self) -> ConfigSave {
                ConfigSave {
                    $($saved_field: self.$saved_field,)*
                }
            }
        }
    }
}

config!{
    saved mouse_sensibility: f32,
    saved fps: u32,
    built dt: f32: 0f32,
}

impl Config {
    pub fn set_fps(&mut self, fps: u32) {
        self.fps = fps;
        self.dt = 1.0 / fps as f32;
    }

    fn from_save(save: ConfigSave) -> Self {
        let mut config = Config::from_save_default(save.clone());
        config.set_fps(save.fps);
        config
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
