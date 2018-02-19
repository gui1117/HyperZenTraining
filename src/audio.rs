use std::fs::File;
use std::path::PathBuf;

use rodio::decoder::Decoder;
use rodio::Source;

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
    endpoint: ::rodio::Endpoint,
    spatial_sinks: Vec<::rodio::SpatialSink>,
    sounds: Vec<::rodio::source::Buffered<Decoder<File>>>,
    left_ear: [f32; 3],
    right_ear: [f32; 3],
}

impl Audio {
    pub fn init() -> Self {
        let sound_filenames = [
            ::CONFIG.shoot_sound.clone(),
            ::CONFIG.kill_sound.clone(),
            ::CONFIG.all_killed_sound.clone(),
            ::CONFIG.portal_sound.clone(),
            ::CONFIG.death_sound.clone(),

        ];

        let mut sounds = vec![];
        for filename in sound_filenames.iter() {
            let mut path = PathBuf::from(::CONFIG.sound_dir.clone());
            path.push(filename);
            let file = File::open(path).unwrap();
            let sound = Decoder::new(file).unwrap().buffered();
            sounds.push(sound);
        }

        Audio {
            endpoint: ::rodio::default_endpoint().unwrap(),
            spatial_sinks: vec![],
            left_ear: [::std::f32::NAN; 3],
            right_ear: [::std::f32::NAN; 3],
            sounds,
        }
    }

    pub fn play_unspatial(&mut self, sound: Sound) {
        let sink = ::rodio::Sink::new(&self.endpoint);
        sink.append(self.sounds[sound as usize].clone());
        sink.detach();
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
        let spatial_sink = ::rodio::SpatialSink::new(
            &self.endpoint,
            pos,
            self.left_ear,
            self.right_ear,
        );
        spatial_sink.append(self.sounds[sound as usize].clone());
        self.spatial_sinks.push(spatial_sink);
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
