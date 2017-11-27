use rand::distributions::{IndependentSample, Range};
use util::ConvCoord;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Conf {
    pub size: usize,
    pub percent: f32,
    pub bug: (isize, isize),
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
    let to_rooms_cells = conf.turrets + conf.avoiders + conf.eraser_avoiders + conf.bouncers + conf.eraser_bouncers;
    let mut rooms_cells;
    loop {
        maze = ::resource::Maze::kruskal(::na::Vector2::new(conf.size as isize, conf.size as isize), conf.percent as f64, ::na::Vector2::new(conf.bug.0, conf.bug.1));
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

    ::entity::create_maze_walls_w(&maze_colors, world);
    ::entity::create_teleport_w(
        ::na::Isometry3::new(
            teleport_end_cell.0.conv(),
            (teleport_end_cell.1 - teleport_end_cell.0).axis_angle_z(),
        ),
        world,
    );

    let dir = teleport_start_cell.1 - teleport_start_cell.0;
    let player_pos = teleport_start_cell.0.conv() - 0.2*::na::Vector3::new(dir[0] as f32, dir[1] as f32, 0.0);
    world.write_resource::<::resource::PlayerControl>().pointer = [
        (-dir[1] as f32).atan2(dir[0] as f32),
        0.0,
    ];
    ::entity::create_player_w(player_pos, world);

    for i in 0..conf.turrets {
        let index = Range::new(0, rooms_cells[i].len()).ind_sample(&mut rng);
        let cell = rooms_cells[i].iter().skip(index).next().unwrap().clone();
        rooms_cells[i].remove(&cell);
        let pos = cell.conv();
        ::entity::create_turret_w(pos, world);
    }

    let mut rooms_cells = rooms_cells.drain(..).flat_map(|r| r.into_iter()).collect::<Vec<_>>();
    for eraser in (0..conf.avoiders).map(|_| false).chain((0..conf.eraser_avoiders).map(|_| true)) {
        let index = Range::new(0, rooms_cells.len()).ind_sample(&mut rng);
        let pos = rooms_cells.swap_remove(index).conv();
        ::entity::create_avoider_w(pos, eraser, world);
    }
    for eraser in (0..conf.bouncers).map(|_| false).chain((0..conf.eraser_bouncers).map(|_| true)) {
        let index = Range::new(0, rooms_cells.len()).ind_sample(&mut rng);
        let pos = rooms_cells.swap_remove(index).conv();
        ::entity::create_bouncer_w(pos, eraser, world);
    }
}
