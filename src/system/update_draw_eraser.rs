use specs::Join;
use std::f32::consts::PI;

pub struct UpdateDynamicDrawEraserSystem;

impl<'a> ::specs::System<'a> for UpdateDynamicDrawEraserSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Aim>,
        ::specs::ReadStorage<'a, ::component::PhysicBody>,
        ::specs::ReadStorage<'a, ::component::Hook>,
        ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
        ::specs::WriteStorage<'a, ::component::DynamicDraw>,
        ::specs::WriteStorage<'a, ::component::Reducer>,
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
            mut reducers,
            physic_world,
        ): Self::SystemData,
    ) {
        for (assets, body) in (&mut dynamic_graphics_assets, &bodies).join() {
            let mut trans = body.get(&physic_world).position() * assets.primitive_trans;
            assets.world_trans = ::graphics::shader::draw1_vs::ty::World {
                world: trans.unwrap().into(),
            }
        }

        for (hook, body, aim) in (&hooks, &bodies, &aims).join() {
            if let Some(ref anchor) = hook.anchor {
                let body_hook_local_pos = ::na::Vector3::new(0.0, 0.2, -0.2);
                let hook_body_pos = body.get(&physic_world).position().translation.vector + aim.rotation*body_hook_local_pos;
                let aimto = hook_body_pos - anchor.pos;

                let assets = dynamic_graphics_assets.get_mut(hook.draw).unwrap();

                let trans = ::na::Isometry3::from_parts(
                    ::na::Translation::from_vector(anchor.pos),
                    ::na::UnitQuaternion::rotation_between(&::na::Vector3::new(0.0, 1.0, 0.0), &aimto).unwrap(),
                )
                    * assets.primitive_trans;

                assets.world_trans = ::graphics::shader::draw1_vs::ty::World {
                    world: trans.unwrap().into(),
                };

                // because we don't want to see the end of the chain we don't draw it when it is
                // viewable
                let angle = ::na::UnitQuaternion::rotation_between(&(aim.rotation*::na::Vector3::new(1.0, 0.0, 0.0)), &aimto).unwrap().angle();
                if angle > PI/3.0 {
                    dynamic_draws.insert(hook.draw, ::component::DynamicDraw);
                } else {
                    dynamic_draws.remove(hook.draw);
                }
            } else {
                dynamic_draws.remove(hook.draw);
            }
        }

        for (assets, reducer) in (&mut dynamic_graphics_assets, &mut reducers).join() {
            let radius = 1.0 - (reducer.timer/reducer.duration);
            let x_radius = if reducer.reduce_x { radius } else { 1.0 };
            let y_radius = if reducer.reduce_y { radius } else { 1.0 };
            let z_radius = if reducer.reduce_z { radius } else { 1.0 };

            let trans = assets.primitive_trans * ::graphics::resizer(x_radius, y_radius, z_radius);
            assets.world_trans = ::graphics::shader::draw1_vs::ty::World {
                world: trans.unwrap().into(),
            };
        }
    }
}
