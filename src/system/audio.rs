use specs::Join;

pub struct AudioSystem;

impl<'a> ::specs::System<'a> for AudioSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::ReadStorage<'a, ::component::Aim>,
        ::specs::ReadStorage<'a, ::component::PhysicBody>,
        ::specs::Fetch<'a, ::resource::PhysicWorld>,
        ::specs::Fetch<'a, ::resource::Save>,
        ::specs::FetchMut<'a, ::resource::Audio>,
    );

    fn run(
        &mut self,
        (players, aims, bodies, physic_world, save, mut audio): Self::SystemData,
    ) {
        let (_, player_aim, player_body) = (&players, &aims, &bodies).join().next().unwrap();
        let position = player_body.get(&physic_world).position().translation.vector;
        audio.update(position, player_aim.rotation.clone(), save.effect_volume(), save.music_volume());
    }
}
