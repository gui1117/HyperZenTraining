use nphysics::object::WorldObject;
use specs::Join;

pub struct PhysicSystem;

impl<'a> ::specs::System<'a> for PhysicSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::ReadStorage<'a, ::component::Momentum>,
     ::specs::WriteStorage<'a, ::component::PhysicBody>,
     ::specs::WriteStorage<'a, ::component::Contactor>,
     ::specs::Fetch<'a, ::resource::Config>,
     ::specs::FetchMut<'a, ::resource::PhysicWorld>);

    fn run(
        &mut self,
        (player, momentums, mut bodies, mut contactors, config, mut physic_world): Self::SystemData,
    ) {
        for (momentum, body) in (&momentums, &mut bodies).join() {
            let body = body.get_mut(&mut physic_world);
            let lin_vel = body.lin_vel();
            let ang_vel = body.ang_vel();

            // TODO: use integrator to modify rigidbody
            body.clear_forces();
            body.append_lin_force(-momentum.damping * lin_vel);
            let direction_force = momentum.force * momentum.direction;
            if let Some(pnt_to_com) = momentum.pnt_to_com {
                let pnt_to_com = body.position().rotation * pnt_to_com;
                body.append_force_wrt_point(direction_force, pnt_to_com);
            } else {
                body.append_lin_force(direction_force);
            }
            body.set_ang_vel_internal(momentum.ang_damping * ang_vel);

            // TODO: gravity if not touching floor
            // body.append_lin_force(10.0*::na::Vector3::new(0.0,0.0,-1.0));
        }
        for contactor in (&mut contactors).join() {
            contactor.contacts.clear();
        }
        for _ in 0..2 {

            physic_world.step(config.dt().clone() / 2.);

            for (co1, co2, mut contact) in physic_world.collision_world().contacts() {
                match (&co1.data, &co2.data) {
                    (&WorldObject::RigidBody(co1), &WorldObject::RigidBody(co2)) => {
                        let body_1 = physic_world.rigid_body(co1);
                        let entity_1 = ::component::PhysicBody::entity(body_1);
                        let body_2 = physic_world.rigid_body(co2);
                        let entity_2 = ::component::PhysicBody::entity(body_2);

                        if let Some(contactor) = contactors.get_mut(entity_1) {
                            contactor.contacts.push((entity_2, contact.clone()));
                        }

                        if let Some(contactor) = contactors.get_mut(entity_2) {
                            contact.flip();
                            contactor.contacts.push((entity_1, contact));
                        }
                    }
                    _ => (),
                }
            }
        }
        for (_, body) in (&player, &mut bodies).join() {
            let body = body.get_mut(&mut physic_world);
            body.set_ang_acc_scale(::na::zero());
            body.set_ang_vel(::na::zero());

            let mut pos = body.position().clone();
            pos = ::na::Isometry3::new(
                ::na::Vector3::new(pos.translation.vector[0], pos.translation.vector[1], 0.5),
                ::na::Vector3::x() * ::std::f32::consts::FRAC_PI_2,
            );
            body.set_transformation(pos);
        }
    }
}
