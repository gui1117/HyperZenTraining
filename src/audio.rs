use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use rodio::decoder::Decoder;
use rodio::Source;
use rodio::Sample;
use show_message::OkOrShow;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum Sound {
    Shoot,
    Kill,
    Death,
    AllKilled,
    Portal,
    Bounce,
    DepthBallAttack,
    DepthBallBirthDeath,
    Eraser,
    Attracted,
}

/// Sounds must be 44100 Hz and stereo
pub struct AudioMix {
    spatial_source: Vec<(::rodio::source::Spatial<SoundSource>, [f32; 3])>,
    unspatial_source: Vec<SoundSource>,
    left_ear: [f32; 3],
    right_ear: [f32; 3],
    delete_indices_cache: Vec<usize>,
    effect_volume: f32,
    eraser_volume: f32,
    eraser_sound_source: InfiniteSoundSource,
}

impl AudioMix {
    fn new(left_ear: [f32; 3], right_ear: [f32; 3], effect_volume: f32) -> Self {
        AudioMix {
            spatial_source: vec![],
            unspatial_source: vec![],
            left_ear,
            right_ear,
            delete_indices_cache: vec![],
            effect_volume: effect_volume,
            eraser_volume: 0.0,
            eraser_sound_source: SOUND_BUFFERS[Sound::Eraser as usize].infinite_source(),
        }
    }

    fn set_listener(&mut self, left_ear: [f32; 3], right_ear: [f32; 3]) {
        self.left_ear = left_ear;
        self.right_ear = right_ear;

        for &mut (ref mut source, position) in &mut self.spatial_source {
            source.set_positions(
                position,
                left_ear,
                right_ear,
            );
        }
    }

    fn add_spatial(&mut self, sound: SoundSource, position: [f32; 3]) {
        assert!(sound.channels() == 2);
        assert!(sound.sample_rate() == 44100);
        let distance_2 = (position[0]-self.left_ear[0]).powi(2)
            + (position[1]-self.left_ear[1]).powi(2)
            + (position[2]-self.left_ear[2]).powi(2);

        if distance_2 < ::CONFIG.max_distance_sound_2 {
            self.spatial_source.push((::rodio::source::Spatial::new(
                sound,
                position,
                self.left_ear,
                self.right_ear,
            ), position));
        }
    }

    fn add_unspatial(&mut self, sound: SoundSource) {
        assert!(sound.channels() == 2);
        assert!(sound.sample_rate() == 44100);
        self.unspatial_source.push(sound);
    }
}

impl Iterator for AudioMix {
    type Item = i16;

    #[inline]
    fn next(&mut self) -> Option<i16> {
        let mut next = self.eraser_sound_source.next().unwrap().amplify(self.eraser_volume);

        self.delete_indices_cache.clear();
        for (i, &mut (ref mut source, _)) in self.spatial_source.iter_mut().enumerate() {
            if let Some(sample) = source.next() {
                next = next.saturating_add(sample);
            } else {
                self.delete_indices_cache.push(i);
            }
        }
        for (i, indice) in self.delete_indices_cache.drain(..).enumerate() {
            self.spatial_source.remove(indice - i);
        }

        self.delete_indices_cache.clear();
        for (i, source) in self.unspatial_source.iter_mut().enumerate() {
            if let Some(sample) = source.next() {
                next = next.saturating_add(sample);
            } else {
                self.delete_indices_cache.push(i);
            }
        }
        for (i, indice) in self.delete_indices_cache.drain(..).enumerate() {
            self.unspatial_source.remove(indice - i);
        }

        Some(next.amplify(self.effect_volume))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl ExactSizeIterator for AudioMix { }

impl Source for AudioMix {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        2
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        44100
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct AudioSinkControl {
    left_ear: [f32; 3],
    right_ear: [f32; 3],
    effect_volume: f32,
    eraser_volume: f32,
    spatial_sounds_to_add: Vec<(Sound, [f32; 3])>,
    sounds_to_add: Vec<Sound>,
}

impl AudioSinkControl {
    fn new(volume: f32) -> Self {
        AudioSinkControl {
            left_ear: [0f32; 3],
            right_ear: [0f32; 3],
            effect_volume: volume,
            eraser_volume: 0.0,
            spatial_sounds_to_add: vec![],
            sounds_to_add: vec![],
        }
    }
}

// 44100 Hz stereo buffer
struct SoundBuffer {
    samples: Arc<Vec<i16>>,
}

impl SoundBuffer {
    fn new(sound: Decoder<Cursor<Vec<u8>>>) -> Result<Self, String> {
        if sound.sample_rate() != 44100 {
            return Err("invalid samples rate: must be 44100 Hz".into());
        }
        if sound.channels() != 2 {
            return Err("invalid channels: must be stereo".into());
        }

        Ok(SoundBuffer {
            samples: Arc::new(sound.collect::<Vec<_>>()),
        })
    }

    fn source(&self) -> SoundSource {
        SoundSource {
            samples: self.samples.clone(),
            cursor: 0,
        }
    }

    fn infinite_source(&self) -> InfiniteSoundSource {
        InfiniteSoundSource {
            samples: self.samples.clone(),
            cursor: 0,
            len: self.samples.len(),
        }
    }
}

// infinite sound soure from a 44100 Hz stereo buffer
struct InfiniteSoundSource {
    samples: Arc<Vec<i16>>,
    cursor: usize,
    len: usize,
}

impl Iterator for InfiniteSoundSource {
    type Item = i16;
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.samples.get(self.cursor).cloned();
        self.cursor = (self.cursor + 1) % self.len;
        sample
    }
}

impl Source for InfiniteSoundSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        2
    }
    fn sample_rate(&self) -> u32 {
        44100
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

// sound soure from a 44100 Hz stereo buffer
struct SoundSource {
    samples: Arc<Vec<i16>>,
    cursor: usize,
}

impl Iterator for SoundSource {
    type Item = i16;
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.samples.get(self.cursor).cloned();
        self.cursor += 1;
        sample
    }
}

impl Source for SoundSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        2
    }
    fn sample_rate(&self) -> u32 {
        44100
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

lazy_static! {
    static ref SOUND_BUFFERS: Vec<SoundBuffer> = {
        let sound_filenames = [
            "assets/sounds/shoot.ogg",
            "assets/sounds/kill.ogg",
            "assets/sounds/death.ogg",
            "assets/sounds/all_killed.ogg",
            "assets/sounds/portal.ogg",
            "assets/sounds/bounce.ogg",
            "assets/sounds/depth_ball_attack.ogg",
            "assets/sounds/depth_ball_birth_death.ogg",
            "assets/sounds/eraser.wav",
            "assets/sounds/attracted.ogg",
        ];

        let mut sound_files = if cfg!(feature = "packed") {
            vec![
                Cursor::new(include_bytes!("../assets/sounds/shoot.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/kill.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/death.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/all_killed.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/portal.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/bounce.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/depth_ball_attack.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/depth_ball_birth_death.ogg").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/eraser.wav").iter().cloned().collect::<Vec<_>>()),
                Cursor::new(include_bytes!("../assets/sounds/attracted.ogg").iter().cloned().collect::<Vec<_>>()),
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

        let mut sound_buffers = vec![];
        for (file, filename) in sound_files.drain(..).zip(sound_filenames.iter()) {
            let sound = Decoder::new(file)
                .ok_or_show(|e| format!("Failed to decode sound {}: {}", filename, e));

            let sound = SoundBuffer::new(sound)
                .ok_or_show(|e| format!("Invalid sound: {}: {}", filename, e));

            sound_buffers.push(sound);
        }
        sound_buffers
    };
}

pub struct Audio {
    audio_sink_control: Option<Arc<Mutex<AudioSinkControl>>>,
    music_sink: Option<::rodio::Sink>,
    // Used to drop sink
    _audio_sink: Option<::rodio::Sink>,
}

impl Audio {
    pub fn init(save: &::resource::Save) -> Self {
        let endpoint = ::rodio::default_output_device();
        if endpoint.is_none() {
            return Audio {
                audio_sink_control: None,
                music_sink: None,
                _audio_sink: None,
            };
        }
        let endpoint = endpoint.unwrap();

        let control = Arc::new(Mutex::new(AudioSinkControl::new(save.effect_volume())));
        let audio_sink_control = Some(control.clone());

        let source = AudioMix::new([0f32; 3], [0f32; 3], save.effect_volume())
            .periodic_access(
                Duration::from_millis(10),
                move |audio_mix| {
                    let mut control = control.lock().unwrap();

                    audio_mix.effect_volume = control.effect_volume;
                    audio_mix.eraser_volume = control.eraser_volume;

                    audio_mix.set_listener(control.left_ear, control.right_ear);

                    for (sound, position) in control.spatial_sounds_to_add.drain(..) {
                        audio_mix.add_spatial(SOUND_BUFFERS[sound as usize].source(), position);
                    }

                    for sound in control.sounds_to_add.drain(..) {
                        audio_mix.add_unspatial(SOUND_BUFFERS[sound as usize].source());
                    }
                }
            );

        let audio_sink = ::rodio::Sink::new(&endpoint);
        audio_sink.append(source);

        let music_filename = "assets/sounds/mm.ogg";
        let music_file = if cfg!(feature = "packed") {
            Cursor::new(include_bytes!("../assets/sounds/mm.ogg").iter().cloned().collect::<Vec<_>>())
        } else {
            let mut buffer = vec![];
            let mut file = File::open(music_filename)
                .ok_or_show(|e| format!("Failed to open sound {}: {}", music_filename, e));
            file.read_to_end(&mut buffer)
                .ok_or_show(|e| format!("Failed to read sound {}: {}", music_filename, e));
            Cursor::new(buffer)
        };

        let music = Decoder::new(music_file)
            .ok_or_show(|e| format!("Failed to decode sound {}: {}", music_filename, e))
            .repeat_infinite();

        let mut music_sink = ::rodio::Sink::new(&endpoint);
        music_sink.set_volume(save.music_volume());
        music_sink.append(::rodio::source::Zero::<i16>::new(2, 44100).take_duration(Duration::from_secs(1)));
        music_sink.append(music);

        Audio {
            _audio_sink: Some(audio_sink),
            music_sink: Some(music_sink),
            audio_sink_control,
        }
    }

    pub fn play_unspatial(&self, sound: Sound) {
        if let Some(ref control) = self.audio_sink_control {
            let mut control = control.lock().unwrap();
            control.sounds_to_add.push(sound);
        }
    }

    pub fn play(&self, sound: Sound, position: [f32; 3]) {
        if let Some(ref control) = self.audio_sink_control {
            let mut control = control.lock().unwrap();
            control.spatial_sounds_to_add.push((sound, position));
        }
    }

    pub fn update(&mut self, position: ::na::Vector3<f32>, aim: ::na::UnitQuaternion<f32>, effect_volume: f32, music_volume: f32, eraser_volume: f32) {
        if let Some(ref control) = self.audio_sink_control {
            let local_left_ear = ::na::Point3::new(0.0, - ::CONFIG.ear_distance/2.0, 0.0);
            let local_right_ear = ::na::Point3::new(0.0, ::CONFIG.ear_distance/2.0, 0.0);

            let world_trans = ::na::Isometry::from_parts(
                ::na::Translation::from_vector(position),
                aim,
            );

            let left_ear = world_trans * local_left_ear;
            let right_ear = world_trans * local_right_ear;

            let mut control = control.lock().unwrap();
            control.effect_volume = effect_volume;
            control.left_ear = left_ear.coords.into();
            control.right_ear = right_ear.coords.into();
            if eraser_volume != control.eraser_volume {
                let current = control.eraser_volume;
                let goal = eraser_volume;
                control.eraser_volume = (current + 0.01*(goal-current).signum()).max(0.0).min(1.0);
            }
        }
        if let Some(ref mut sink) = self.music_sink {
            sink.set_volume(music_volume);
        }
    }
}
