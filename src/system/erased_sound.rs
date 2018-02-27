pub struct ErasedSoundSystem;

impl<'a> ::specs::System<'a> for ErasedSoundSystem {
    type SystemData = (
        ::specs::Fetch<'a, ::resource::Audio>,
        ::specs::Fetch<'a, ::resource::Graphics>,
        ::specs::FetchMut<'a, ::resource::ErasedStatus>,
    );

    fn run(&mut self, (audio, graphics, mut erased_status): Self::SystemData) {
        let sum: f32 = graphics.erased_buffer.read().unwrap()
            .iter()
            .sum();

        let new_erase_amount = ::graphics::GROUP_COUNTER_SIZE as f32 - sum;

//         if new_number_erased > 0 {
//             println!("todo");
//             // audio.play_unspatial(::audio::Sound::AllKilled);
//             // number_erased.0 = new_number_erased;
//         }
    }
}
