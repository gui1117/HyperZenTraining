pub struct ErasedSoundSystem;

impl<'a> ::specs::System<'a> for ErasedSoundSystem {
    type SystemData = (
        ::specs::Fetch<'a, ::resource::Audio>,
        ::specs::Fetch<'a, ::resource::Graphics>,
        ::specs::FetchMut<'a, ::resource::ErasedStatus>,
    );

    fn run(&mut self, (audio, graphics, mut erased_status): Self::SystemData) {
        let new_number_erased = graphics.erased_buffer.read().unwrap()
            .iter()
            .filter(|elt| **elt == 0.0)
            .count();

//         if new_number_erased > 0 {
//             println!("todo");
//             // audio.play_unspatial(::audio::Sound::AllKilled);
//             // number_erased.0 = new_number_erased;
//         }
    }
}
