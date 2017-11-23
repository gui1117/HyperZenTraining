use std::f32::consts::FRAC_PI_2;

pub fn create_maze_walls<'a>(
    bodies: &mut ::specs::WriteStorage<'a, ::component::PhysicBody>,
    static_draws: &mut ::specs::WriteStorage<'a, ::component::StaticDraw>,
    physic_world: &mut ::specs::FetchMut<'a, ::resource::PhysicWorld>,
    graphics: &::specs::Fetch<'a, ::resource::Graphics>,
    maze: &::specs::Fetch<'a, ::resource::Maze>,
    entities: &::specs::Entities,
) {
    super::create_floor_ceil(
        0.0,
        true,
        bodies,
        static_draws,
        physic_world,
        graphics,
        entities,
    );
    super::create_floor_ceil(
        1.0,
        false,
        bodies,
        static_draws,
        physic_world,
        graphics,
        entities,
    );

    let groups = ::graphics::Primitive::Plane.reserve(maze.size[0].max(maze.size[1]) as usize * 2 + 2);
    let compute_group = |x, o| (2*x + o) as usize;

    // actually 0 and last are not necessary if it is circled and it must be
    for x in 1..maze.size[0] {
        for y in 1..maze.size[1] {
            let pos = ::na::Vector3::new(x as f32 + 0.5, y as f32 + 0.5, 0.5);
            if maze.wall(x-1, y) != maze.wall(x, y) {
                let pos = ::na::Isometry3::new(
                    pos + ::na::Vector3::new(-0.5, 0.0, 0.0),
                    ::na::Vector3::y() * FRAC_PI_2,
                );
                let groups = groups[compute_group(x, 0)].clone();
                super::create_wall_side(
                    pos,
                    groups,
                    bodies,
                    static_draws,
                    physic_world,
                    graphics,
                    entities,
                );
            }
            if maze.wall(x, y-1) != maze.wall(x, y) {
                let pos = ::na::Isometry3::new(
                    pos + ::na::Vector3::new(0.0, -0.5, 0.0),
                    ::na::Vector3::x() * FRAC_PI_2,
                );
                let groups = groups[compute_group(y, 1)].clone();
                super::create_wall_side(
                    pos,
                    groups,
                    bodies,
                    static_draws,
                    physic_world,
                    graphics,
                    entities,
                );
            }
        }
    }
}
