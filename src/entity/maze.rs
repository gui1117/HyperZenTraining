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

    maze.assert_square();
    let size = maze.size[0];

    for x in 0..size {
        let mut up_coords = None;
        let mut down_coords = None;
        for y in 0..size + 1 {
            let up_wall = if x == 0 || y == size {
                false
            } else {
                maze.wall(x - 1, y)
            };
            let wall = if y == size { false } else { maze.wall(x, y) };
            let down_wall = if x + 1 == size || y == size {
                false
            } else {
                maze.wall(x + 1, y)
            };

            let up_side_wall = wall && !up_wall;
            let down_side_wall = wall && !down_wall;

            if up_side_wall && up_coords.is_none() {
                up_coords = Some((y, y));
            } else if up_side_wall && up_coords.is_some() {
                up_coords.as_mut().unwrap().1 = y;
            } else if !up_side_wall && up_coords.is_some() {
                let c = up_coords.take().unwrap();
                let x_radius = 0.5;
                let y_radius = (c.1 - c.0 + 1) as f32 / 2.0;
                let pos = ::na::Isometry3::new(
                    ::na::Vector3::new(x as f32, c.0 as f32 + y_radius, 0.5),
                    ::na::Vector3::y() * FRAC_PI_2,
                );
                super::create_wall_side(
                    pos,
                    x_radius,
                    y_radius,
                    bodies,
                    static_draws,
                    physic_world,
                    graphics,
                    entities,
                );
            }

            if down_side_wall && down_coords.is_none() {
                down_coords = Some((y, y));
            } else if down_side_wall && down_coords.is_some() {
                down_coords.as_mut().unwrap().1 = y;
            } else if !down_side_wall && down_coords.is_some() {
                let c = down_coords.take().unwrap();
                let x_radius = 0.5;
                let y_radius = (c.1 - c.0 + 1) as f32 / 2.0;
                let pos = ::na::Isometry3::new(
                    ::na::Vector3::new(x as f32 + 1.0, c.0 as f32 + y_radius, 0.5),
                    ::na::Vector3::y() * FRAC_PI_2,
                );
                super::create_wall_side(
                    pos,
                    x_radius,
                    y_radius,
                    bodies,
                    static_draws,
                    physic_world,
                    graphics,
                    entities,
                );
            }
        }
    }

    for y in 0..size {
        let mut up_coords = None;
        let mut down_coords = None;
        for x in 0..size + 1 {
            let up_wall = if y == 0 || x == size {
                false
            } else {
                maze.wall(x, y - 1)
            };
            let wall = if x == size { false } else { maze.wall(x, y) };
            let down_wall = if y + 1 == size || x == size {
                false
            } else {
                maze.wall(x, y + 1)
            };

            let up_side_wall = wall && !up_wall;
            let down_side_wall = wall && !down_wall;

            if up_side_wall && up_coords.is_none() {
                up_coords = Some((x, x));
            } else if up_side_wall && up_coords.is_some() {
                up_coords.as_mut().unwrap().1 = x;
            } else if !up_side_wall && up_coords.is_some() {
                let c = up_coords.take().unwrap();
                let x_radius = (c.1 - c.0 + 1) as f32 / 2.0;
                let y_radius = 0.5;
                let pos = ::na::Isometry3::new(
                    ::na::Vector3::new(c.0 as f32 + x_radius, y as f32, 0.5),
                    ::na::Vector3::x() * FRAC_PI_2,
                );
                super::create_wall_side(
                    pos,
                    x_radius,
                    y_radius,
                    bodies,
                    static_draws,
                    physic_world,
                    graphics,
                    entities,
                );
            }

            if down_side_wall && down_coords.is_none() {
                down_coords = Some((x, x));
            } else if down_side_wall && down_coords.is_some() {
                down_coords.as_mut().unwrap().1 = x;
            } else if !down_side_wall && down_coords.is_some() {
                let c = down_coords.take().unwrap();
                let x_radius = (c.1 - c.0 + 1) as f32 / 2.0;
                let y_radius = 0.5;
                let pos = ::na::Isometry3::new(
                    ::na::Vector3::new(c.0 as f32 + x_radius, y as f32 + 1.0, 0.5),
                    ::na::Vector3::x() * FRAC_PI_2,
                );
                super::create_wall_side(
                    pos,
                    x_radius,
                    y_radius,
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
