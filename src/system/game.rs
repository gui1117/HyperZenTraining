use rand::distributions::{IndependentSample, Range};
use specs::Join;
use util::ConvCoord;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct GameSystem {
    init: bool,
}

impl GameSystem {
    pub fn new() -> Self {
        GameSystem { init: false }
    }
    pub fn run(&mut self, world: &mut ::specs::World) {
        if !self.init {
            self.init = true;
            self.create_level(world);
        }
        if (
            &world.read::<::component::Teleport>(),
            &world.read::<::component::Proximitor>(),
        ).join()
            .any(|(_, p)| !p.intersections.is_empty())
        {
            world.delete_all();
            self.create_level(world);
        }
    }

//     fn _create_2D_kill_all_level(&self, world: &mut ::specs::World, size: usize, turrets: usize, avoiders: usize, bouncers: usize, black_avoiders: usize, black_bouncers: usize) {
//         // TODO:
//         // turrets are in larger rooms
//         unimplemented!();
//     }

//     fn _create_2D_walkthrough_level(&self, world: &mut ::specs::World, size: usize, turrets: usize, avoiders: usize, bouncers: usize, black_avoider_percent: f32, black_bouncer_percent: f32) {
//         // TODO:
//         // turrets are in larger rooms
//         unimplemented!();
//     }

    fn create_level(&self, world: &mut ::specs::World) {
        let mut rng = ::rand::thread_rng();
        let mut maze;
        let mut cells_digged;
        let mut rooms_cells;
        loop {
            maze = ::resource::Maze::kruskal(::na::Vector2::new(11, 11), 20.0, ::na::zero());
            println!("{}", maze);
            maze.reduce(1);
            println!("{}", maze);
            maze.circle();
            println!("{}", maze);
            maze.fill_smallests();
            println!("{}", maze);
            while maze.fill_dead_corridors() {}
            println!("{}", maze);
            maze.extend(1);
            maze.circle();
            let dead_rooms = maze.compute_dead_room_and_corridor_zones();
            let dead_rooms_cells: HashSet<_> = dead_rooms.iter().flat_map(|r| r.iter()).collect();

            let to_dig = 1;
            cells_digged = maze.dig_cells(to_dig, |cell| !dead_rooms_cells.contains(cell));
            if cells_digged.len() != to_dig {
                continue;
            }

            rooms_cells = maze.compute_room_zones();
            rooms_cells.sort_unstable_by(|r1, r2| r2.len().cmp(&r1.len()));
            for room in &mut rooms_cells {
                room.retain(|cell| !maze.is_neighbouring_corridor(cell));
            }
            rooms_cells.retain(|r| !r.is_empty());
            if rooms_cells.is_empty() {
                continue;
            }
            break;
        }
        println!("{}", maze);

        let mut maze_colors = HashMap::new();

        let teleport_cell = cells_digged.first().unwrap();
        maze_colors.insert(teleport_cell.0, ::graphics::color::GREEN);

        world.add_resource(::resource::GameEvents(vec![]));
        world.add_resource(::resource::PhysicWorld::new());
        world.add_resource(maze);
        world.add_resource(::resource::DepthCoef(1.0));

        ::entity::create_maze_walls(
            &maze_colors,
            &mut world.write(),
            &mut world.write(),
            &mut world.write_resource(),
            &world.read_resource(),
            &world.read_resource(),
            &world.read_resource(),
        );
        ::entity::create_teleport(
            ::na::Isometry3::new(
                teleport_cell.0.conv(),
                (teleport_cell.1 - teleport_cell.0).axis_angle(),
            ),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write_resource(),
            &world.read_resource(),
        );
        ::entity::create_turret(
            {
                let index = Range::new(0, rooms_cells[0].len()).ind_sample(&mut rng);
                let cell = rooms_cells[0].iter().skip(index).next().unwrap().clone();
                rooms_cells[0].remove(&cell);
                cell.conv()
            },
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write_resource(),
            &world.read_resource(),
        );
        // ::entity::create_avoider(
        //     world
        //         .read_resource::<::resource::Maze>()
        //         .random_free_float(),
        //     false,
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write_resource(),
        //     &world.read_resource());
        // ::entity::create_bouncer(
        //     world
        //         .read_resource::<::resource::Maze>()
        //         .random_free_float(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write_resource(),
        //     &world.read_resource());
        ::entity::create_player(
            world
                .read_resource::<::resource::Maze>()
                .random_free()
                .conv(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write_resource(),
            &world.read_resource(),
        );

        world.maintain();
    }
}
