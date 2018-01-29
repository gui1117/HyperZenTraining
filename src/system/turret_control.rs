use specs::Join;

pub struct TurretControlSystem {
    shoots: Vec<(::na::Vector3<f32>, ::na::Vector3<f32>)>,
}

impl TurretControlSystem {
    pub fn new() -> Self {
        TurretControlSystem {
            shoots: vec![],
        }
    }
}

impl<'a> ::specs::System<'a> for TurretControlSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::WriteStorage<'a, ::component::Turret>,
        ::specs::WriteStorage<'a, ::component::PhysicBody>,
        ::specs::WriteStorage<'a, ::component::DepthBall>,
        ::specs::WriteStorage<'a, ::component::Contactor>,
        ::specs::WriteStorage<'a, ::component::DynamicDraw>,
        ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
        ::specs::WriteStorage<'a, ::component::Life>,
        ::specs::WriteStorage<'a, ::component::Momentum>,
        ::specs::Fetch<'a, ::resource::UpdateTime>,
        ::specs::FetchMut<'a, ::resource::PhysicWorld>,
        ::specs::Entities<'a>,
    );

    fn run(
        &mut self,
        (
            players,
            mut turrets,
            mut bodies,
            mut depth_balls,
            mut contactors,
            mut dynamic_draws,
            mut dynamic_graphics_assets,
            mut lifes,
            mut momentums,
            update_time,
            mut physic_world,
            entities,
        ): Self::SystemData,
    ) {
        let player_pos = {
            let (_, player_body) = (&players, &bodies).join().next().unwrap();
            player_body.get(&physic_world).position().clone()
        };

        for (turret, body) in (&mut turrets, &mut bodies).join() {
            turret.last_shoot += update_time.0;
            let direction = player_pos.translation.vector - turret.position;

            while turret.last_shoot > turret.reload_time {
                turret.last_shoot -= turret.reload_time;
                self.shoots.push((turret.position, direction));
            }

            let rotation = ::na::UnitQuaternion::rotation_between(
                &::na::Vector3::new(0.0, 0.0, 1.0),
                &direction,
            ).unwrap_or(::na::UnitQuaternion::new(::na::zero()));

            let trans = ::na::Isometry3::from_parts(
                ::na::Translation::from_vector(turret.position),
                rotation,
            );
            body.get_mut(&mut physic_world).set_transformation(trans);
        }

        for (pos, dir) in self.shoots.drain(..) {
            ::entity::create_depth_ball(
                pos,
                dir,
                &mut momentums,
                &mut depth_balls,
                &mut contactors,
                &mut bodies,
                &mut dynamic_draws,
                &mut dynamic_graphics_assets,
                &mut lifes,
                &mut physic_world,
                &entities,
            );
        }
    }
}
