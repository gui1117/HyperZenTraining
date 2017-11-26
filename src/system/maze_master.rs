use specs::Join;
use util::ConvCoord;
use rand::distributions::{IndependentSample, Range};

pub struct MazeMasterSystem;

impl<'a> ::specs::System<'a> for MazeMasterSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::WriteStorage<'a, ::component::PhysicBody>,
     ::specs::WriteStorage<'a, ::component::Momentum>,
     ::specs::WriteStorage<'a, ::component::Avoider>,
     ::specs::WriteStorage<'a, ::component::Bouncer>,
     ::specs::WriteStorage<'a, ::component::DynamicEraser>,
     ::specs::WriteStorage<'a, ::component::DynamicDraw>,
     ::specs::WriteStorage<'a, ::component::DynamicGraphicsAssets>,
     ::specs::WriteStorage<'a, ::component::Life>,
     ::specs::WriteStorage<'a, ::component::Contactor>,
     ::specs::Fetch<'a, ::resource::Maze>,
     ::specs::FetchMut<'a, ::resource::PhysicWorld>,
     ::specs::Fetch<'a, ::resource::Config>,
     ::specs::Entities<'a>);

    fn run(
        &mut self,
        (players, mut bodies, mut momentums, mut avoiders, mut bouncers, mut dynamic_erasers, mut dynamic_draws, mut dynamic_graphics_assets, mut lives, mut contactors, maze, mut physic_world, config, entities): Self::SystemData,
){
        let avoider_population = 10;
        let bouncer_population = 10;
        let kill_distance = 15.0;
        let spawn_distance = 10;

        let player_pos = {
            let p = (&players, &bodies)
                .join()
                .last()
                .unwrap()
                .1
                .get(&physic_world)
                .position_center();
            ::na::Vector3::new(p[0], p[1], p[2])
        };

        // kill too far entities
        {
            let kill_too_far = |body: &::component::PhysicBody, life: &mut ::component::Life| {
                let pos = body.get(&physic_world).position().translation.vector;
                if (pos - player_pos).norm() > kill_distance {
                    // Or maybe directly delete the entity
                    // here at least if the delete is seen
                    // then it can look intentional
                    life.kill()
                }
            };

            for (_, body, life) in (&bouncers, &bodies, &mut lives).join() {
                kill_too_far(body, life);
            }
            for (_, body, life) in (&avoiders, &bodies, &mut lives).join() {
                kill_too_far(body, life);
            }
        }

        let avoider_len = avoiders.join().fold(0, |acc, _| acc + 1);
        let bouncer_len = bouncers.join().fold(0, |acc, _| acc + 1);

        if avoider_len == avoider_population && bouncer_len == bouncer_population {
            return;
        }

        let square = maze.free_in_square(
            ::na::Vector2::new(player_pos[0] as isize, player_pos[1] as isize),
            spawn_distance,
        );
        if square.is_empty() {
            panic!("maze is too small to be able to create entities");
        }
        let square_range = Range::new(0, square.len());
        let mut rng = ::rand::thread_rng();

        for _ in avoider_len..avoider_population {
            let pos = square[square_range.ind_sample(&mut rng)];

            ::entity::create_avoider(
                pos.conv(),
                false,
                &mut momentums,
                &mut avoiders,
                &mut bodies,
                &mut dynamic_erasers,
                &mut dynamic_draws,
                &mut dynamic_graphics_assets,
                &mut lives,
                &mut physic_world,
                &config,
                &entities,
            );
        }

        for _ in bouncer_len..bouncer_population {
            let pos = square[square_range.ind_sample(&mut rng)];

            ::entity::create_bouncer(
                pos.conv(),
                false,
                &mut momentums,
                &mut bouncers,
                &mut bodies,
                &mut dynamic_erasers,
                &mut dynamic_draws,
                &mut dynamic_graphics_assets,
                &mut lives,
                &mut contactors,
                &mut physic_world,
                &config,
                &entities,
            );
        }
    }
}
