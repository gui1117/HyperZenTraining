use std::fs::File;
use std::io::Cursor;
use std::io::Read;

use rodio::decoder::Decoder;
use rodio::Source;
use show_message::OkOrShow;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum Sound {
    Shoot,
    Kill,
    Death,
    AllKilled,
    Portal,
}

pub struct Audio {
    endpoint: Option<::rodio::Endpoint>,
    spatial_sinks: Vec<::rodio::SpatialSink>,
    sounds: Vec<::rodio::source::Buffered<Decoder<Cursor<Vec<u8>>>>>,
    left_ear: [f32; 3],
    right_ear: [f32; 3],
}

impl Audio {
    pub fn init() -> Self {
        let sound_filenames = [
            "assets/sounds/shoot.ogg",
            "assets/sounds/kill.ogg",
            "assets/sounds/death.ogg",
            "assets/sounds/all_killed.ogg",
            "assets/sounds/portal.ogg",
        ];

        let mut sound_files = if cfg!(feature = "packed") {
            vec![
                Cursor::new(include_bytes!("../assets/sounds/shoot.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/kill.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/death.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/all_killed.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/portal.ogg").iter().cloned().collect::<Vec<_>>()),
            ]
        } else {
            sound_filenames.iter()
                .map(|s| {
                    let mut buffer = vec![];
                    let mut file = File::open(s)
                        .ok_or_show(|e| format!("Failed to open sound {}: {}", s, e));
                    file.read_to_end(&mut buffer)
                        .ok_or_show(|e| format!("Failed to read sound {}: {}", s, e));
                    Cursor::new(buffer)
                })
                .collect::<Vec<_>>()
        };

        let mut sounds = vec![];
        for (file, filename) in sound_files.drain(..).zip(sound_filenames.iter()) {
            let sound = Decoder::new(file)
                .ok_or_show(|e| format!("Failed to decode sound {}: {}", filename, e))
                .buffered();
            sounds.push(sound);
        }

        Audio {
            endpoint: ::rodio::default_endpoint(),
            spatial_sinks: vec![],
            left_ear: [::std::f32::NAN; 3],
            right_ear: [::std::f32::NAN; 3],
            sounds,
        }
    }

    pub fn play_unspatial(&mut self, sound: Sound) {
        if let Some(ref endpoint) = self.endpoint {
            let sink = ::rodio::Sink::new(endpoint);
            sink.append(self.sounds[sound as usize].clone());
            sink.detach();
        }
    }


    pub fn play_on_emitter(&mut self, sound: Sound) {
        let pos = [
            (self.left_ear[0] + self.right_ear[0])/2.0,
            (self.left_ear[1] + self.right_ear[1])/2.0,
            (self.left_ear[2] + self.right_ear[2])/2.0,
        ];
        self.play(sound, pos);
    }

    pub fn play(&mut self, sound: Sound, pos: [f32; 3]) {
        if let Some(ref endpoint) = self.endpoint {
            let spatial_sink = ::rodio::SpatialSink::new(
                endpoint,
                pos,
                self.left_ear,
                self.right_ear,
                );
            spatial_sink.append(self.sounds[sound as usize].clone());
            self.spatial_sinks.push(spatial_sink);
        }
    }

    pub fn set_emitter(&mut self, position: ::na::Vector3<f32>, aim: ::na::UnitQuaternion<f32>) {
        let local_left_ear = ::na::Point3::new(0.0, - ::CONFIG.ear_distance/2.0, 0.0);
        let local_right_ear = ::na::Point3::new(0.0, ::CONFIG.ear_distance/2.0, 0.0);

        let world_trans = ::na::Isometry::from_parts(
            ::na::Translation::from_vector(position),
            aim,
        );

        let left_ear = world_trans * local_left_ear;
        let right_ear = world_trans * local_right_ear;

        self.left_ear = left_ear.coords.into();
        self.right_ear = right_ear.coords.into();
        self.spatial_sinks.retain(|s| !s.empty());
        for spatial_sink in &mut self.spatial_sinks {
            spatial_sink.set_left_ear_position(self.left_ear);
            spatial_sink.set_right_ear_position(self.right_ear);
        }
    }
}
