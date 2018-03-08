use specs::Join;

pub struct AudioSystem;

impl<'a> ::specs::System<'a> for AudioSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::ReadStorage<'a, ::component::Aim>,
        ::specs::ReadStorage<'a, ::component::PhysicBody>,
        ::specs::Fetch<'a, ::resource::PhysicWorld>,
        ::specs::Fetch<'a, ::resource::Save>,
        ::specs::Fetch<'a, ::resource::ErasedStatus>,
        ::specs::FetchMut<'a, ::resource::Audio>,
    );

    fn run(
        &mut self,
        (players, aims, bodies, physic_world, save, erased_status, mut audio): Self::SystemData,
    ) {
        let (_, player_aim, player_body) = (&players, &aims, &bodies).join().next().unwrap();
        let position = player_body.get(&physic_world).position().translation.vector;
        audio.update(position, player_aim.rotation.clone(), save.effect_volume(), save.music_volume(), erased_status.amount*20000.0);
    }
}
