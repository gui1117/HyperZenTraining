use specs::Join;

pub struct UpdateDynamicDrawEraserSystem;

impl<'a> ::specs::System<'a> for UpdateDynamicDrawEraserSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Aim>,
        ::specs::ReadStorage<'a, ::component::PhysicBody>,
        ::specs::ReadStorage<'a, ::component::Hook>,
        ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
        ::specs::WriteStorage<'a, ::component::DynamicDraw>,
        ::specs::Fetch<'a, ::resource::PhysicWorld>,
    );

    fn run(
        &mut self,
        (
            aims,
            bodies,
            hooks,
            mut dynamic_graphics_assets,
            mut dynamic_draws,
            physic_world,
        ): Self::SystemData,
    ) {
        for (assets, body) in (&mut dynamic_graphics_assets, &bodies).join() {
            let trans = body.get(&physic_world).position() * assets.primitive_trans;
            assets.world_trans = ::graphics::shader::draw1_vs::ty::World {
                world: trans.unwrap().into(),
            }
        }

        for (hook, body, aim) in (&hooks, &bodies, &aims).join() {
            if let Some(ref anchor) = hook.anchor {
                // TODO: insert only when seing it
                dynamic_draws.insert(hook.draw, ::component::DynamicDraw);
                let assets = dynamic_graphics_assets.get_mut(hook.draw).unwrap();

                // remove or add draw depending of aim
                let trans = ::na::Isometry3::new(
                    anchor.pos,
                    // TODO: angle with body + "aim" + translation
                    ::na::zero(),
                )
                    * assets.primitive_trans;

                assets.world_trans = ::graphics::shader::draw1_vs::ty::World {
                    world: trans.unwrap().into(),
                };
            } else {
                dynamic_draws.remove(hook.draw);
            }
        }
    }
}
