use resource::Trend::*;

pub struct ErasedSoundSystem;

impl<'a> ::specs::System<'a> for ErasedSoundSystem {
    type SystemData = (
        ::specs::Fetch<'a, ::resource::Audio>,
        ::specs::Fetch<'a, ::resource::Graphics>,
        ::specs::FetchMut<'a, ::resource::ErasedStatus>,
    );

    fn run(&mut self, (audio, graphics, mut erased_status): Self::SystemData) {
        if let Ok(o) = graphics.erased_amount_buffer.read() {
            println!("{}", o[0]);
        }
        // let erased_amount_buffer = loop {
        //     let res = graphics.erased_amount_buffer.read();
        //     if let Ok(o) = res {
        //         break o;
        //     }
        //     println!("tot");
        // };
        // let new_amount: u32 = erased_amount_buffer[0];

        // println!("amount: {}", new_amount);

        // let new_trend = if new_amount == erased_status.amount {
        //     Stable
        // } else if new_amount > erased_status.amount {
        //     Increase
        // } else {
        //     Decrease
        // };

        // match (new_trend, erased_status.trend) {
        //     (Increase, Stable) | (Increase, Decrease) => {
        //         audio.play_unspatial(::audio::Sound::EraserIncrease)
        //     }
        //     (Decrease, Stable) | (Decrease, Increase) => {
        //         audio.play_unspatial(::audio::Sound::EraserDecrease)
        //     }
        //     _ => (),
        // }

        // erased_status.amount = new_amount;
        // erased_status.trend = new_trend;
    }
}
