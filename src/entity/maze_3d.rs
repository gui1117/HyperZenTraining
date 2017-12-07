use std::collections::HashMap;
use util::ConvCoord;

pub fn create_3d_maze_walls_w(
    colors: &HashMap<::na::Vector3<isize>, ::graphics::Color>,
    maze: &::maze::Maze<::na::U3>,
    world: &::specs::World,
) {
    create_3d_maze_walls(
        &colors,
        maze,
        &mut world.write(),
        &mut world.write(),
        &mut world.write_resource(),
        &world.read_resource(),
        &world.read_resource(),
        &world.read_resource(),
    );
}

pub fn create_3d_maze_walls<'a>(
    colors: &HashMap<::na::Vector3<isize>, ::graphics::Color>,
    maze: &::maze::Maze<::na::U3>,
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    static_draws: &mut ::specs::WriteStorage<'a, ::component::StaticDraw>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    graphics: &::specs::Fetch<'a, ::resource::Graphics>,
    config: &::specs::Fetch<'a, ::resource::Config>,
    entities: &::specs::Entities,
) {
    let index = |x, y, z, o| {
        match o {
            0 => x as usize * 3,
            1 => y as usize * 3 + o,
            2 => z as usize * 3 + o,
            _ => unreachable!(),
        }
    };
    let groups = ::graphics::Primitive::Plane.reserve(maze.size.iter().max().unwrap().clone() as usize * 3 + 3);

    for cell in &maze.walls {
        ::entity::create_wall_cube_physic(maze.to_world(cell), maze.scale/2.0, bodies, physic_world, entities);
        for dl in &maze.neighbours {
            let neighbour = cell + dl;
            if maze.walls.contains(&neighbour) {
                continue;
            }

            let color = colors.get(&neighbour).cloned();
            let orientation = dl.iter().enumerate().find(|&(_, &n)| n != 0).map(|(i, _)| i).unwrap();
            let groups = if color.is_some() {
                ::graphics::Primitive::Plane.reserve(1).pop().unwrap()
            } else {
                groups[index(cell[0], cell[1], cell[2], orientation)].clone()
            };
            let dl_f32 = ::na::Vector3::new(dl[0] as f32, dl[1] as f32, dl[2] as f32) * maze.scale/2.;
            let pos = ::na::Isometry3::new(maze.to_world(cell) + dl_f32, dl.axis_angle_z());

            ::entity::create_wall_side_draw(pos, maze.scale/2., color, groups, static_draws, graphics, config, entities);
        }
    }
}
