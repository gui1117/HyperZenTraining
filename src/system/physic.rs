use nphysics::object::WorldObject;
use ncollide::events::ProximityEvent;
use ncollide::query::Proximity;
use specs::Join;

pub struct PhysicSystem;

impl<'a> ::specs::System<'a> for PhysicSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::ReadStorage<'a, ::component::Momentum>,
     ::specs::ReadStorage<'a, ::component::AirMomentum>,
     ::specs::WriteStorage<'a, ::component::PhysicBody>,
     ::specs::WriteStorage<'a, ::component::Contactor>,
     ::specs::WriteStorage<'a, ::component::Proximitor>,
     ::specs::Fetch<'a, ::resource::Config>,
     ::specs::FetchMut<'a, ::resource::PhysicWorld>,
     ::specs::Entities<'a>);

    fn run(
        &mut self,
        (players, momentums, air_momentums, mut bodies, mut contactors, mut proximitors, config, mut physic_world, entities): Self::SystemData,
    ) {
        // TODO: use integrator to modify rigidbody
        for (momentum, body, entity) in (&momentums, &mut bodies, &*entities).join() {
            let body = body.get_mut(&mut physic_world);
            let lin_vel = body.lin_vel();
            let ang_vel = body.ang_vel();

            body.clear_forces();

            match (contactors.get(entity), air_momentums.get(entity)) {
                (Some(contactor), Some(air_momentum)) => {
                    if contactor.contacts.iter().any(|&(_, ref c)| c.normal[2] < -0.8) {
                        body.append_lin_force(-momentum.damping * lin_vel);
                    } else {
                        body.append_lin_force(air_momentum.gravity_force * ::na::Vector3::new(0.0, 0.0, -1.0));
                        body.append_lin_force(-air_momentum.damping * lin_vel);
                    }
                },
                _ => body.append_lin_force(-momentum.damping * lin_vel),
            }
            let direction_force = momentum.force * momentum.direction;
            if let Some(pnt_to_com) = momentum.pnt_to_com {
                let pnt_to_com = body.position().rotation * pnt_to_com;
                body.append_force_wrt_point(direction_force, pnt_to_com);
            } else {
                body.append_lin_force(direction_force);
            }
            if let Some(ang_force) = momentum.ang_force {
                body.append_ang_force(ang_force);
            }
            body.set_ang_vel_internal(momentum.ang_damping * ang_vel);
        }
        for contactor in (&mut contactors).join() {
            contactor.contacts.clear();
        }
        for proximitor in (&mut proximitors).join() {
            proximitor.intersections.clear();
        }
        for _ in 0..2 {

            physic_world.step(config.dt().clone() / 2.);

            for (co1, co2, mut contact) in physic_world.collision_world().contacts() {
                let (entity_1, entity_2) = match (&co1.data, &co2.data) {
                    (&WorldObject::RigidBody(w1), &WorldObject::RigidBody(w2)) => {
                        let e1 = physic_world.rigid_body(w1);
                        let e2 = physic_world.rigid_body(w2);
                        (
                            ::component::PhysicBody::entity(e1),
                            ::component::PhysicBody::entity(e2),
                        )
                    }
                    _ => unreachable!(),
                };

                if let Some(contactor) = contactors.get_mut(entity_1) {
                    contactor.contacts.push((entity_2, contact.clone()));
                }

                if let Some(contactor) = contactors.get_mut(entity_2) {
                    contact.flip();
                    contactor.contacts.push((entity_1, contact));
                }
            }

            for event in physic_world.collision_world().proximity_events() {
                if let &ProximityEvent {
                    co1,
                    co2,
                    new_status: Proximity::Intersecting,
                    ..
                } = event
                {
                    let co1 = physic_world
                        .collision_world()
                        .collision_object(co1)
                        .unwrap();
                    let co2 = physic_world
                        .collision_world()
                        .collision_object(co2)
                        .unwrap();
                    // we can't just get e1 and e2 and check for each if there is a proximitor
                    // because the rigid body of eX may be involve in a proximity even if the
                    // proximitor is associated to eX sensor
                    match (&co1.data, &co2.data) {
                        (&WorldObject::Sensor(w1), &WorldObject::RigidBody(w2)) => {
                            let e1 = ::component::PhysicSensor::entity(physic_world.sensor(w1));
                            let e2 = ::component::PhysicBody::entity(physic_world.rigid_body(w2));
                            if let Some(proximitor) = proximitors.get_mut(e1) {
                                proximitor.intersections.push(e2);
                            }
                        }
                        (&WorldObject::RigidBody(w1), &WorldObject::Sensor(w2)) => {
                            let e1 = ::component::PhysicBody::entity(physic_world.rigid_body(w1));
                            let e2 = ::component::PhysicSensor::entity(physic_world.sensor(w2));
                            if let Some(proximitor) = proximitors.get_mut(e2) {
                                proximitor.intersections.push(e1);
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        for (_, body) in (&players, &mut bodies).join() {
            let body = body.get_mut(&mut physic_world);
            body.set_ang_acc_scale(::na::zero());
            body.set_ang_vel(::na::zero());

            let mut pos = body.position().clone();
            pos = ::na::Isometry3::new(
                pos.translation.vector,
                ::na::Vector3::x() * ::std::f32::consts::FRAC_PI_2,
            );
            body.set_transformation(pos);
        }
    }
}
