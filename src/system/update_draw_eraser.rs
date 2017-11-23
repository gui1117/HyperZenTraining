use specs::Join;
use alga::general::SubsetOf;

pub struct UpdateDynamicDrawEraserSystem;

impl<'a> ::specs::System<'a> for UpdateDynamicDrawEraserSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::PhysicBody>,
     ::specs::ReadStorage<'a, ::component::WeaponAnchor>,
     ::specs::ReadStorage<'a, ::component::WeaponAnimation>,
     ::specs::ReadStorage<'a, ::component::Aim>,
     ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>);

fn run(&mut self, (bodies, weapon_anchors, weapon_animations, aims, mut dynamic_graphics_assets, physic_world): Self::SystemData){
        for (assets, body) in (&mut dynamic_graphics_assets, &bodies).join() {
            let trans = body.get(&physic_world).position() * assets.primitive_trans;
            assets.world_trans =
                ::graphics::shader::draw1_vs::ty::World { world: trans.unwrap().into() }
        }

        for (assets, anchor) in (&mut dynamic_graphics_assets, &weapon_anchors).join() {
            let animation = weapon_animations.get(anchor.anchor).unwrap();
            let body = bodies.get(anchor.anchor).unwrap();
            let aim = aims.get(anchor.anchor).unwrap();

            let aim_trans = {
                let ah: ::na::Transform3<f32> =
                    ::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -aim.x_dir)).to_superset();
                let av: ::na::Transform3<f32> = ::na::Rotation3::new(
                    ::na::Vector3::new(0.0, -aim.dir[2].asin(), 0.0),
                ).to_superset();
                ah * av
            };

            let trans = body.get(&physic_world).position().translation * aim_trans *
                animation.weapon_trans * assets.primitive_trans;
            assets.world_trans =
                ::graphics::shader::draw1_vs::ty::World { world: trans.unwrap().into() }
        }
    }
}
