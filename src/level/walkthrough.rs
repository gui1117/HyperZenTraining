use rand::distributions::{IndependentSample, Range};
use util::ConvCoord;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Conf {
    pub size: (usize, usize),
    pub percent: f32,
    pub bug: (isize, isize),
    pub scale: f32,
    pub turrets: usize,

    pub avoider_generators: usize,
    pub avoider_salvo: usize,
    pub avoider_eraser_proba: f32,
    pub avoider_time_between_salvo: f32,

    pub bouncer_generators: usize,
    pub bouncer_salvo: usize,
    pub bouncer_eraser_proba: f32,
    pub bouncer_time_between_salvo: f32,
}

pub fn create(world: &mut ::specs::World, conf: &Conf) {
    let mut rng = ::rand::thread_rng();
    let mut maze;
    let to_dig = 2 + conf.avoider_generators + conf.bouncer_generators;
    let mut cells_digged;
    let to_rooms_cells = conf.turrets;
    let mut rooms_cells;
    loop {
        maze = ::maze::Maze::kruskal(::na::Vector2::new(conf.size.0 as isize, conf.size.1 as isize), conf.percent as f64, ::na::Vector2::new(conf.bug.0, conf.bug.1), conf.scale);
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

    let mut avoider_generator_cells = vec![];
    for _ in 0..conf.avoider_generators {
        let cell = cells_digged.pop().unwrap();
        maze_colors.insert(cell.0, ::graphics::Color::Red);
        avoider_generator_cells.push(cell.0)
    }

    let mut bouncer_generator_cells = vec![];
    for _ in 0..conf.bouncer_generators {
        let cell = cells_digged.pop().unwrap();
        maze_colors.insert(cell.0, ::graphics::Color::Red);
        bouncer_generator_cells.push(cell.0)
    }

    ::entity::create_2d_maze_walls_w(&maze_colors, &maze, world);

    ::entity::create_teleport_w(
        ::na::Isometry3::new(
            maze.to_world(&teleport_end_cell.0),
            (teleport_end_cell.1 - teleport_end_cell.0).axis_angle_z(),
        ),
        world,
    );

    let dir = teleport_start_cell.1 - teleport_start_cell.0;
    let player_pos = maze.to_world(&teleport_start_cell.0) - 0.2*::na::Vector3::new(dir[0] as f32, dir[1] as f32, 0.0);
    world.write_resource::<::resource::PlayerControl>().pointer = [
        (-dir[1] as f32).atan2(dir[0] as f32),
        0.0,
    ];
    ::entity::create_player_w(player_pos, world);

    for i in 0..conf.turrets {
        let index = Range::new(0, rooms_cells[i].len()).ind_sample(&mut rng);
        let cell = rooms_cells[i].iter().skip(index).next().unwrap().clone();
        rooms_cells[i].remove(&cell);
        let pos = maze.to_world(&cell);
        ::entity::create_turret_w(pos, world);
    }

    avoider_generator_cells.iter().map(|c| (c, ::component::GeneratedEntity::Avoider, conf.avoider_salvo, conf.avoider_time_between_salvo, conf.avoider_eraser_proba))
        .chain(bouncer_generator_cells.iter().map(|c| (c, ::component::GeneratedEntity::Bouncer, conf.bouncer_salvo, conf.bouncer_time_between_salvo, conf.bouncer_eraser_proba)))
        .for_each(|t| ::entity::create_generator(maze.to_world(&t.0), t.1, t.2, t.3, t.4, &mut world.write(), &world.read_resource()));

    world.add_resource(::resource::Maze::Maze2D(maze));
}
