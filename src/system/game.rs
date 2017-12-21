use nphysics::resolution::{AccumulatedImpulseSolver, CorrectionMode};

pub struct GameSystem {
    current_level: Option<usize>,
}

impl GameSystem {
    pub fn new() -> Self {
        GameSystem {
            current_level: None,
        }
    }
    pub fn run(&mut self, world: &mut ::specs::World) {
        let recreate_level = match self.current_level {
            Some(ref mut current_level) => {
                let end = world.read_resource::<::resource::EndLevel>().0;
                if end {
                    *current_level += 1;
                }
                end
            }
            None => {
                self.current_level = Some(0);
                true
            }
        };

        if recreate_level {
            let current_level = self.current_level.unwrap();

            let physic_world = {
                let config = world.read_resource::<::resource::Config>();
                let mut physic_world = ::resource::PhysicWorld::new();
                *physic_world.constraints_solver() = AccumulatedImpulseSolver::new(
                    config.accumulated_impulse_solver_step,
                    CorrectionMode::VelocityAndPosition(
                        config.correction_mode_a,
                        config.correction_mode_b,
                        config.correction_mode_c,
                    ),
                    config.accumulated_impulse_solver_joint_corr_factor,
                    config.accumulated_impulse_solver_rest_eps,
                    config.accumulated_impulse_solver_num_first_order_iter,
                    config.accumulated_impulse_solver_num_second_order_iter,
                );
                physic_world
            };

            world.delete_all();
            world.add_resource(::resource::GameEvents(vec![]));
            world.add_resource(::resource::DepthCoef(1.0));
            world.add_resource(::resource::EndLevel(false));
            world.add_resource(physic_world);

            let level = world.read_resource::<::resource::Config>().levels[current_level].clone();
            level.create(world);

            world.maintain();
        }
    }
}
