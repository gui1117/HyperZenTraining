use specs::Join;

pub struct AvoiderControlSystem;

impl<'a> ::specs::System<'a> for AvoiderControlSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::ReadStorage<'a, ::component::Aim>,
     ::specs::ReadStorage<'a, ::component::PhysicBody>,
     ::specs::WriteStorage<'a, ::component::Avoider>,
     ::specs::WriteStorage<'a, ::component::Momentum>,
     ::specs::Fetch<'a, ::resource::PhysicWorld>,
     ::specs::Fetch<'a, ::resource::Config>,
     ::specs::Fetch<'a, ::resource::Maze>);

    fn run(
        &mut self,
        (players, aims, bodies, mut avoiders, mut momentums, physic_world, config, maze): Self::SystemData,
    ) {
        let (_, player_aim, player_body) = (&players, &aims, &bodies).join().next().unwrap();
        let player_aim_dir = player_aim.rotation * ::na::Vector3::x();

        let player_pos = player_body.get(&physic_world).position().clone();

        for (avoider, momentum, body) in (&mut avoiders, &mut momentums, &bodies).join() {
            let avoider_pos = body.get(&physic_world).position().clone();

            let recompute_goal = if let Some(goal) = avoider.goal {
                (avoider_pos.translation.vector -
                     ::na::Vector3::new(
                        goal[0] as f32 + 0.5,
                        goal[1] as f32 + 0.5,
                        avoider_pos.translation.vector[2],
                    )).norm() < 0.5
            } else {
                if (avoider_pos.translation.vector - player_pos.translation.vector).norm() < 1.0 {
                    avoider.goal.take();
                    false
                } else {
                    true
                }
            };

            if recompute_goal {
                let pos = ::na::Vector2::new(
                    avoider_pos.translation.vector[0] as isize,
                    avoider_pos.translation.vector[1] as isize,
                );
                let goal = ::na::Vector2::new(
                    player_pos.translation.vector[0] as isize,
                    player_pos.translation.vector[1] as isize,
                );
                // TODO: it crash sometimes here because it doesn't find path
                avoider.goal = maze.find_path(pos, goal).unwrap().get(1).cloned();
            }

            let (goal_direction, goal_coef) = {
                let goal_pos = if let Some(goal) = avoider.goal {
                    ::na::Vector3::new(
                        goal[0] as f32 + 0.5,
                        goal[1] as f32 + 0.5,
                        avoider_pos.translation.vector[2],
                    )
                } else {
                    player_pos.translation.vector
                };

                (
                    (goal_pos - avoider_pos.translation.vector).normalize(),
                    1f32,
                )
            };

            let (avoid_direction, avoid_coef) = {
                let avoider_pos_rel_player = avoider_pos.translation.vector -
                    player_pos.translation.vector;
                let avoid_vector = avoider_pos_rel_player -
                    avoider_pos_rel_player.dot(&player_aim_dir) * player_aim_dir;
                if avoid_vector.norm() != 0.0 {
                    let avoid_norm = avoid_vector.norm();
                    let avoid_direction = avoid_vector.normalize();
                    if avoid_norm > config.avoider_avoid_norm {
                        (avoid_direction, 0f32)
                    } else {
                        // TODO: coefficent
                        (avoid_direction, 1f32) //1.0/avoid_norm)
                    }
                } else {
                    let random = ::na::Vector3::new_random();
                    // TODO: coefficient
                    (
                        (random - random.dot(&player_aim_dir) * player_aim_dir).normalize(),
                        1f32,
                    )
                }
            };

            momentum.direction = (goal_coef * goal_direction + avoid_coef * avoid_direction)
                .normalize();
        }
    }
}
