use rand::Rand;
use specs::Join;
use component::GeneratedEntity;

pub struct GeneratorSystem;

impl<'a> ::specs::System<'a> for GeneratorSystem {
    type SystemData = (
        ::specs::WriteStorage<'a, ::component::Generator>,
        ::specs::WriteStorage<'a, ::component::PhysicBody>,
        ::specs::WriteStorage<'a, ::component::Momentum>,
        ::specs::WriteStorage<'a, ::component::Avoider>,
        ::specs::WriteStorage<'a, ::component::Bouncer>,
        ::specs::WriteStorage<'a, ::component::DynamicEraser>,
        ::specs::WriteStorage<'a, ::component::DynamicDraw>,
        ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
        ::specs::WriteStorage<'a, ::component::Life>,
        ::specs::WriteStorage<'a, ::component::Contactor>,
        ::specs::FetchMut<'a, ::resource::PhysicWorld>,
        ::specs::Fetch<'a, ::resource::Config>,
        ::specs::Entities<'a>,
    );

    fn run(
        &mut self,
        (
            mut generators,
            mut bodies,
            mut momentums,
            mut avoiders,
            mut bouncers,
            mut dynamic_erasers,
            mut dynamic_draws,
            mut dynamic_graphics_assets,
            mut lives,
            mut contactors,
            mut physic_world,
            config,
            entities,
        ): Self::SystemData,
    ) {
        let mut rng = ::rand::thread_rng();

        for mut generator in (&mut generators).join() {
            generator.timer -= config.dt();
            if generator.timer <= 0.0 {
                generator.timer = generator.time_between_salvo;
                for _ in 0..generator.salvo {
                    match generator.entity {
                        GeneratedEntity::Bouncer => ::entity::create_bouncer(
                            generator.pos,
                            f32::rand(&mut rng) < generator.eraser_probability,
                            &mut momentums,
                            &mut bouncers,
                            &mut bodies,
                            &mut dynamic_erasers,
                            &mut dynamic_draws,
                            &mut dynamic_graphics_assets,
                            &mut lives,
                            &mut contactors,
                            &mut physic_world,
                            &config,
                            &entities,
                        ),
                        GeneratedEntity::Avoider => ::entity::create_avoider(
                            generator.pos,
                            f32::rand(&mut rng) < generator.eraser_probability,
                            &mut momentums,
                            &mut avoiders,
                            &mut bodies,
                            &mut dynamic_erasers,
                            &mut dynamic_draws,
                            &mut dynamic_graphics_assets,
                            &mut lives,
                            &mut physic_world,
                            &config,
                            &entities,
                        ),
                    }
                }
            }
        }
    }
}
