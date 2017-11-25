use specs::Join;
use util::ConvCoord;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct GameSystem {
    init: bool,
}

impl GameSystem {
    pub fn new() -> Self {
        GameSystem {
            init: false,
        }
    }
    pub fn run(&mut self, world: &mut ::specs::World) {
        if !self.init {
            self.init = true;
            self.create_level(world);
        }
        if (&world.read::<::component::Teleport>(), &world.read::<::component::Proximitor>()).join().any(|(_, p)| !p.intersections.is_empty()) {
            world.delete_all();
            self.create_level(world);
        }
    }

    fn create_level(&self, world: &mut ::specs::World) {
        let mut maze;
        let mut cells_digged;
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
            if cells_digged.len() == to_dig {
                break;
            }
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
        // ::entity::create_turret(
        //     world
        //         .read_resource::<::resource::Maze>()
        //         .random_free()
        //         .conv(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write(),
        //     &mut world.write_resource(),
        //     &world.read_resource(),
        // );
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
