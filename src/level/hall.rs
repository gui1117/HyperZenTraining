use std::collections::HashMap;
use std::f32::consts::FRAC_PI_2;

pub fn create_hall(world: &mut ::specs::World) {
    let number_of_levels = ::CONFIG.levels.len() as isize;
    let levels_on_top = number_of_levels / 2;
    let levels_on_side =  number_of_levels - levels_on_top;

    let size = ::na::Vector2::new(levels_on_top.max(1)*3+3, levels_on_side*3+3);
    let mut maze = ::maze::Maze::new_rectangle(size, 1.0);
    maze.circle();
    maze.extend(1);
    maze.circle();

    let maze_size = maze.size.clone();

    let mut maze_colors = HashMap::new();

    // Build Player
    let start_cell = ::na::Vector2::new(maze_size[0] - 3, maze_size[1] - 2);
    maze_colors.insert(start_cell, (::CONFIG.start_color, false));
    maze.walls.remove(&start_cell);
    let dir = ::na::Vector3::new(0.0, -1.0, 0.0);
    let player_pos = maze.to_world(&start_cell)
        - 0.2 * ::na::Vector3::new(dir[0] as f32, dir[1] as f32, 0.0);
    world.write_resource::<::resource::PlayerControl>().pointer =
        [(-dir[1] as f32).atan2(dir[0] as f32), 0.0];
    ::entity::create_player_w(player_pos, false, world);

    // Build Teleport
    let teleport_cells = (0isize..levels_on_top)
        .map(|i| (::na::Vector2::new(maze_size[0] - 4 - i*3, 1), ::na::Vector3::new(-FRAC_PI_2, 0.0, 0.0)))
        .chain((0..levels_on_side)
            .map(|i| (::na::Vector2::new(1, i*3+3), ::na::Vector3::new(0.0, FRAC_PI_2, 0.0))));

    for (i, (teleport_cell, teleport_dir)) in teleport_cells.enumerate() {
        let activated = i == 0 || world.read_resource::<::resource::Save>().score(i-1).is_some();

        maze_colors.insert(teleport_cell, (::CONFIG.end_color, activated));
        maze.walls.remove(&teleport_cell);

        let score_pos = if teleport_dir[1] == 0.0 {
            ::na::Isometry3::new(
                maze.to_world(&teleport_cell)+::na::Vector3::new(-0.7, 0.5, 0.3),
                ::na::Vector3::new(::std::f32::consts::FRAC_PI_2, 0.0, 0.0)
            ) * ::na::UnitQuaternion::new(::na::Vector3::new(0.0, ::std::f32::consts::PI, 0.0))
        } else {
            ::na::Isometry3::new(
                maze.to_world(&teleport_cell)+::na::Vector3::new(0.5, 0.7, 0.3),
                ::na::Vector3::new(::std::f32::consts::FRAC_PI_2, 0.0, 0.0)
            ) * ::na::UnitQuaternion::new(::na::Vector3::new(0.0, ::std::f32::consts::FRAC_PI_2, 0.0))
        };

        ::entity::draw_score(
            score_pos,
            i,
            world,
        );

        let number_pos = if teleport_dir[1] == 0.0 {
            ::na::Isometry3::new(
                maze.to_world(&teleport_cell)+::na::Vector3::new(0.0, -0.5, 0.0),
                ::na::Vector3::new(::std::f32::consts::FRAC_PI_2, 0.0, 0.0)
            ) * ::na::UnitQuaternion::new(::na::Vector3::new(0.0, ::std::f32::consts::PI, 0.0))
        } else {
            ::na::Isometry3::new(
                maze.to_world(&teleport_cell)+::na::Vector3::new(-0.5, 0.0, 0.0),
                ::na::Vector3::new(::std::f32::consts::FRAC_PI_2, 0.0, 0.0)
            ) * ::na::UnitQuaternion::new(::na::Vector3::new(0.0, ::std::f32::consts::FRAC_PI_2, 0.0))
        };

        ::entity::draw_number(
            number_pos,
            format!("{}", i+1),
            world,
        );

        if activated {
            ::entity::create_teleport_w(
                ::na::Isometry3::new(
                    maze.to_world(&teleport_cell),
                    teleport_dir,
                ),
                maze.scale,
                ::resource::LevelAction::Level(i),
                world,
            );
        }
    }

    // Build Maze
    ::entity::create_2d_maze_walls_w(&maze_colors, &maze, world);
    world.add_resource(::resource::Maze::Maze2D(maze));
}
