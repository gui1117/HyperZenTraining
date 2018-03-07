use resource::Trend::*;

pub struct ErasedSoundSystem;

impl<'a> ::specs::System<'a> for ErasedSoundSystem {
    type SystemData = (
        ::specs::Fetch<'a, ::resource::Audio>,
        ::specs::Fetch<'a, ::resource::Graphics>,
        ::specs::FetchMut<'a, ::resource::ErasedStatus>,
    );

    fn run(&mut self, (audio, graphics, mut erased_status): Self::SystemData) {
        let new_amount = 0f32;
        // TODO:
        // let new_amount = graphics.erased_buffer.read().unwrap()
        //     .iter()
        //     .sum();

        let new_trend = if new_amount == erased_status.amount {
            Stable
        } else if new_amount > erased_status.amount {
            Increase
        } else {
            Decrease
        };

        match (new_trend, erased_status.trend) {
            (Increase, Stable) | (Increase, Decrease) => {
                audio.play_unspatial(::audio::Sound::EraserIncrease)
            }
            (Decrease, Stable) | (Decrease, Increase) => {
                audio.play_unspatial(::audio::Sound::EraserDecrease)
            }
            _ => (),
        }

        erased_status.amount = new_amount;
        erased_status.trend = new_trend;
    }
}
