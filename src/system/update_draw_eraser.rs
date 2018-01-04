use specs::Join;

pub struct UpdateDynamicDrawEraserSystem;

impl<'a> ::specs::System<'a> for UpdateDynamicDrawEraserSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::PhysicBody>,
        ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
        ::specs::Fetch<'a, ::resource::PhysicWorld>,
    );

    fn run(
        &mut self,
        (
            bodies,
            mut dynamic_graphics_assets,
            physic_world,
        ): Self::SystemData,
    ) {
        for (assets, body) in (&mut dynamic_graphics_assets, &bodies).join() {
            let trans = body.get(&physic_world).position() * assets.primitive_trans;
            assets.world_trans = ::graphics::shader::draw1_vs::ty::World {
                world: trans.unwrap().into(),
            }
        }
    }
}
