use nphysics::resolution::{AccumulatedImpulseSolver, CorrectionMode};

pub struct GameSystem {
    current_level: Option<Level>,
}

#[derive(Clone, Copy)]
enum Level {
    Hall,
    Level(usize, usize),
}

impl GameSystem {
    pub fn new() -> Self {
        GameSystem {
            current_level: None,
        }
    }
    pub fn run(&mut self, world: &mut ::specs::World) {
        let action = {
            let mut level_actions = world.write_resource::<::resource::LevelActions>();
            let action = level_actions.0.first().cloned();
            level_actions.0.clear();
            action
        };

        let recreate_level = match (self.current_level, action) {
            (None, _) => Some(Level::Hall),
            (Some(Level::Hall), Some(::resource::LevelAction::Level(level))) => {
                if ::CONFIG.levels[level].len() != 0 {
                    Some(Level::Level(level, 0))
                } else {
                    //TODO: update scores
                    Some(Level::Hall)
                }
            },
            (Some(Level::Level(level, part)), Some(::resource::LevelAction::Next)) => {
                if ::CONFIG.levels[level].len() > part + 1 {
                    Some(Level::Level(level, part+1))
                } else {
                    //TODO: update scores
                    Some(Level::Hall)
                }
            },
            (current_level, Some(::resource::LevelAction::Reset)) => current_level,
            (_, Some(::resource::LevelAction::ReturnHall)) => Some(Level::Hall),
            (Some(_), None) => None,
            (Some(Level::Hall), Some(::resource::LevelAction::Next)) => {
                println!("INTERNAL ERROR: called next in hall");
                Some(Level::Hall)
            },
            (Some(Level::Level(..)), Some(::resource::LevelAction::Level(..))) => {
                println!("INTERNAL ERROR: called go to level outside hall");
                Some(Level::Hall)
            },
        };

        if let Some(level) = recreate_level {
            self.current_level = Some(level);

            let physic_world = {
                let mut physic_world = ::resource::PhysicWorld::new();
                *physic_world.constraints_solver() = AccumulatedImpulseSolver::new(
                    ::CONFIG.accumulated_impulse_solver_step,
                    CorrectionMode::VelocityAndPosition(
                        ::CONFIG.correction_mode_a,
                        ::CONFIG.correction_mode_b,
                        ::CONFIG.correction_mode_c,
                    ),
                    ::CONFIG.accumulated_impulse_solver_joint_corr_factor,
                    ::CONFIG.accumulated_impulse_solver_rest_eps,
                    ::CONFIG.accumulated_impulse_solver_num_first_order_iter,
                    ::CONFIG.accumulated_impulse_solver_num_second_order_iter,
                );
                physic_world
            };

            world.delete_all();
            world.add_resource(::resource::Events(vec![]));
            world.add_resource(::resource::DepthCoef(1.0));
            world.add_resource(physic_world);

            match level {
                Level::Hall => ::level::create_hall(world),
                Level::Level(level, part) => ::CONFIG.levels[level][part].create(world),
            }

            world.maintain();
        }
    }
}
