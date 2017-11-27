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
            world.delete_all();
            world.add_resource(::resource::GameEvents(vec![]));
            world.add_resource(::resource::PhysicWorld::new());
            world.add_resource(::resource::DepthCoef(1.0));
            world.add_resource(::resource::EndLevel(false));

            let level = world.read_resource::<::resource::Config>().levels[current_level].clone();
            level.create(world);

            world.maintain();
        }
    }
}
