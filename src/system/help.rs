use specs::Join;

pub struct HelpSystem;

impl<'a> ::specs::System<'a> for HelpSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Attracted>,
        ::specs::ReadStorage<'a, ::component::Avoider>,
        ::specs::ReadStorage<'a, ::component::Bouncer>,
        ::specs::ReadStorage<'a, ::component::Motionless>,
        ::specs::FetchMut<'a, ::resource::Help>,
    );

    fn run(&mut self, (attracted, avoider, bouncer, motionless, mut help): Self::SystemData) {
        let r = [
            (attracted.join().count(), "Attracted"),
            (avoider.join().count(), "Avoider"),
            (bouncer.join().count(), "Bouncer"),
            (motionless.join().count(), "Motionless"),
        ];

        let remaining = r
            .iter()
            .filter(|(count, _)| *count != 0)
            .collect::<Vec<_>>();

        if remaining.len() == 0 {
            help.0 = "Go to the portal".into();
        } else {
            help.0 = String::from("Remains:");
            for (count, name) in remaining {
                help.0.push_str(&format!("\n- {}: {}", name, count));
            }
        }
    }
}
