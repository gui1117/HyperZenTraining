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

    fn create_level(&self, world: &mut ::specs::World) {
        world.add_resource(::resource::GameEvents(vec![]));
        world.add_resource(::resource::PhysicWorld::new());
        world.add_resource(::resource::DepthCoef(1.0));

        self.create_2d_walkthrough_level(world, 21, 40.0, ::na::zero(), 0, 1, 0, 0, 0);

        world.maintain();
    }

//     fn _create_2D_kill_all_level(&self, world: &mut ::specs::World, size: usize, turrets: usize, avoiders: usize, bouncers: usize, black_avoiders: usize, black_bouncers: usize) {
//         // TODO:
//         // turrets are in larger rooms
//         unimplemented!();
//     }

    fn create_2d_walkthrough_level(&self, world: &mut ::specs::World, size: usize, percent: f32, bug: ::na::Vector2<isize>, turrets: usize, avoiders: usize, bouncers: usize, black_avoiders: usize, black_bouncers: usize) {
        let mut rng = ::rand::thread_rng();
        let mut maze;
        let to_dig = 2;
        let mut cells_digged;
        let to_rooms_cells = turrets + avoiders + black_avoiders + bouncers + black_bouncers;
        let mut rooms_cells;
        loop {
            maze = ::resource::Maze::kruskal(::na::Vector2::new(size as isize, size as isize), percent as f64, bug);
            maze.reduce(1);
            maze.circle();
            maze.fill_smallests();
            while maze.fill_dead_corridors() {}
            maze.extend(1);
            maze.circle();
            let dead_rooms = maze.compute_dead_room_and_corridor_zones();
            let dead_rooms_cells: HashSet<_> = dead_rooms.iter().flat_map(|r| r.iter()).collect();

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
            if rooms_cells.len() < to_rooms_cells {
                continue;
            }
            break;
        }
        println!("{}", maze);

        let mut maze_colors = HashMap::new();

        let teleport_end_cell = cells_digged.pop().unwrap();
        maze_colors.insert(teleport_end_cell.0, ::graphics::Color::Green);
        let teleport_start_cell = cells_digged.pop().unwrap();
        maze_colors.insert(teleport_start_cell.0, ::graphics::Color::Red);

        world.add_resource(maze);

        ::entity::create_maze_walls(
            &maze_colors,
            &mut world.write(),
            &mut world.write(),
            &mut world.write_resource(),
            &world.read_resource(),
            &world.read_resource(),
            &world.read_resource(),
            &world.read_resource(),
        );
        ::entity::create_teleport(
            ::na::Isometry3::new(
                teleport_end_cell.0.conv(),
                (teleport_end_cell.1 - teleport_end_cell.0).axis_angle(),
            ),
            &mut world.write(),
            &mut world.write(),
            &mut world.write(),
            &mut world.write_resource(),
            &world.read_resource(),
            &world.read_resource(),
        );
        ::entity::create_player(
            ::na::Isometry3::new(
                teleport_start_cell.0.conv(),
                (teleport_start_cell.1 - teleport_start_cell.0).axis_angle(),
            ),
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
            &world.read_resource(),
        );
        for i in 0..turrets {
            ::entity::create_turret(
                {
                    let index = Range::new(0, rooms_cells[i].len()).ind_sample(&mut rng);
                    let cell = rooms_cells[i].iter().skip(index).next().unwrap().clone();
                    rooms_cells[i].remove(&cell);
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
                &world.read_resource(),
            );
        }

        let mut rooms_cells = rooms_cells.drain(..).flat_map(|r| r.into_iter()).collect::<Vec<_>>();
        for black in (0..avoiders).map(|_| false).chain((0..black_avoiders).map(|_| true)) {
            ::entity::create_avoider(
                {
                    let index = Range::new(0, rooms_cells.len()).ind_sample(&mut rng);
                    rooms_cells.swap_remove(index).conv()
                },
                black,
                &mut world.write(),
                &mut world.write(),
                &mut world.write(),
                &mut world.write(),
                &mut world.write(),
                &mut world.write(),
                &mut world.write(),
                &mut world.write_resource(),
                &world.read_resource(),
                &world.read_resource()
            );
        }
        for black in (0..bouncers).map(|_| false).chain((0..black_bouncers).map(|_| true)) {
            ::entity::create_bouncer(
                {
                    let index = Range::new(0, rooms_cells.len()).ind_sample(&mut rng);
                    rooms_cells.swap_remove(index).conv()
                },
                black,
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
                &world.read_resource()
            );
        }
    }
}
