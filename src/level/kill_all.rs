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
    pub avoiders: usize,
    pub bouncers: usize,
    pub eraser_avoiders: usize,
    pub eraser_bouncers: usize,
}

pub fn create(world: &mut ::specs::World, conf: &Conf) {
    let mut rng = ::rand::thread_rng();
    let mut maze;
    let to_dig = 2;
    let mut cells_digged;
    let to_rooms_cells =
        conf.turrets + conf.avoiders + conf.eraser_avoiders + conf.bouncers + conf.eraser_bouncers;
    let mut rooms_cells;
    loop {
        println!("try create a maze");
        maze = ::maze::Maze::kruskal(
            ::na::Vector2::new(conf.size.0 as isize, conf.size.1 as isize),
            conf.percent as f64,
            ::na::Vector2::new(conf.bug.0, conf.bug.1),
            conf.scale,
        );
        maze.reduce(1);
        maze.circle();
        maze.fill_smallests();
        while maze.fill_dead_corridors() {}
        maze.extend(1);
        maze.circle();
        let dead_rooms = maze.compute_dead_room_and_corridor_zones();
        let dead_rooms_cells: HashSet<_> = dead_rooms.iter().flat_map(|r| r.iter()).collect();
        println!("maze: {}", maze);

        cells_digged = maze.dig_cells(to_dig, |cell| !dead_rooms_cells.contains(cell));
        if cells_digged.len() != to_dig {
            println!("failed to dig enough");
            continue;
        }

        println!("cell digged maze: {}", maze);
        rooms_cells = maze.compute_room_zones();
        rooms_cells.sort_unstable_by(|r1, r2| r2.len().cmp(&r1.len()));
        for room in &mut rooms_cells {
            room.retain(|cell| !maze.is_neighbouring_corridor(cell));
        }
        rooms_cells.retain(|r| !r.is_empty());
        if rooms_cells.iter().flat_map(|r| r.iter()).count() < to_rooms_cells {
            println!("failed to have enough rooms cell");
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

    ::entity::create_2d_maze_walls_w(&maze_colors, &maze, world);

    ::entity::create_teleport_w(
        ::na::Isometry3::new(
            maze.to_world(&teleport_end_cell.0),
            (teleport_end_cell.1 - teleport_end_cell.0).axis_angle_z(),
        ),
        maze.scale,
        world,
    );

    let dir = teleport_start_cell.1 - teleport_start_cell.0;
    let player_pos = maze.to_world(&teleport_start_cell.0)
        - 0.2 * ::na::Vector3::new(dir[0] as f32, dir[1] as f32, 0.0);
    world.write_resource::<::resource::PlayerControl>().pointer =
        [(-dir[1] as f32).atan2(dir[0] as f32), 0.0];
    ::entity::create_player_w(player_pos, false, world);

    for i in 0..conf.turrets {
        let index = Range::new(0, rooms_cells[i].len()).ind_sample(&mut rng);
        let cell = rooms_cells[i].iter().skip(index).next().unwrap().clone();
        rooms_cells[i].remove(&cell);
        let pos = maze.to_world(&cell);
        ::entity::create_turret_w(pos, world);
    }

    let mut rooms_cells = rooms_cells
        .drain(..)
        .flat_map(|r| r.into_iter())
        .collect::<Vec<_>>();
    for eraser in (0..conf.avoiders)
        .map(|_| false)
        .chain((0..conf.eraser_avoiders).map(|_| true))
    {
        let index = Range::new(0, rooms_cells.len()).ind_sample(&mut rng);
        let pos = maze.to_world(&rooms_cells.swap_remove(index));
        ::entity::create_avoider_w(pos, eraser, world);
    }
    for eraser in (0..conf.bouncers)
        .map(|_| false)
        .chain((0..conf.eraser_bouncers).map(|_| true))
    {
        let index = Range::new(0, rooms_cells.len()).ind_sample(&mut rng);
        let pos = maze.to_world(&rooms_cells.swap_remove(index));
        ::entity::create_bouncer_w(pos, eraser, world);
    }

    world.add_resource(::resource::Maze::Maze2D(maze));
}
