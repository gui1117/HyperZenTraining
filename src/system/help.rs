use specs::Join;

pub struct HelpSystem;

impl<'a> ::specs::System<'a> for HelpSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Attracted>,
        ::specs::ReadStorage<'a, ::component::Avoider>,
        ::specs::ReadStorage<'a, ::component::Bouncer>,
        ::specs::ReadStorage<'a, ::component::Motionless>,
        ::specs::FetchMut<'a, ::resource::Help>,
        ::specs::Fetch<'a, ::resource::Text>,
    );

    fn run(&mut self, (attracted, avoider, bouncer, motionless, mut help, text): Self::SystemData) {
        let r = [
            (attracted.join().count(), &text.attracted),
            (avoider.join().count(), &text.avoider),
            (bouncer.join().count(), &text.bouncer),
            (motionless.join().count(), &text.motionless),
        ];

        let remaining = r
            .iter()
            .filter(|(count, _)| *count != 0)
            .collect::<Vec<_>>();

        if remaining.len() == 0 {
            help.0 = text.go_to_portal.clone();
        } else {
            help.0 = text.remains.clone();
            for (count, name) in remaining {
                help.0.push_str(&format!("\n  {} - {}", name, count));
            }
        }
    }
}
