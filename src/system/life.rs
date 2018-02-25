use specs::Join;

pub struct LifeSystem;

impl<'a> ::specs::System<'a> for LifeSystem {
    type SystemData = (
        ::specs::WriteStorage<'a, ::component::PhysicBody>,
        ::specs::WriteStorage<'a, ::component::DynamicDraw>,
        ::specs::WriteStorage<'a, ::component::DynamicEraser>,
        ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
        ::specs::WriteStorage<'a, ::component::Life>,
        ::specs::WriteStorage<'a, ::component::Reducer>,
        ::specs::FetchMut<'a, ::resource::PhysicWorld>,
        ::specs::FetchMut<'a, ::resource::Audio>,
        ::specs::Entities<'a>,
    );

    fn run(
        &mut self,
        (mut bodies, mut dynamic_draws, mut dynamic_erasers, mut dynamic_graphics_assets, mut lives, mut reducers, mut physic_world, mut audio, entities): Self::SystemData,
    ) {
        use component::Life;
        for (life, entity) in (&mut lives, &*entities).join() {
            match *life {
                Life::EraserDead => {
                    *life = Life::DrawAlive;
                    dynamic_draws.insert(entity, ::component::DynamicDraw);
                    dynamic_erasers.remove(entity).unwrap();
                }
                Life::DrawDead => {
                    let body = bodies.get_mut(entity).unwrap();

                    audio.play_unspatial(::audio::Sound::Kill);

                    let death_animation_assets = {
                        let assets = dynamic_graphics_assets.get(entity).unwrap();
                        let position = body.get(&physic_world).position();
                        ::component::DynamicGraphicsAssets::new(
                            assets.primitive,
                            assets.groups.clone(),
                            assets.color,
                            position * assets.primitive_trans,
                        )
                    };

                    let death_animation_entity = entities.create();
                    dynamic_draws.insert(death_animation_entity, ::component::DynamicDraw);
                    dynamic_graphics_assets.insert(death_animation_entity, death_animation_assets);
                    reducers.insert(death_animation_entity, ::component::Reducer::new(::CONFIG.death_duration, true, true, true));

                    body.remove(&mut physic_world);
                    entities.delete(entity).unwrap();
                }
                _ => (),
            }
        }
    }
}
