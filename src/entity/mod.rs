pub const WALL_GROUP: usize = 1; // Both vertical walls and ceil and floor
pub const FLOOR_CEIL_GROUP: usize = 2;
pub const ALIVE_GROUP: usize = 3;
pub const MONSTER_GROUP: usize = 4;
pub const TURRET_GROUP: usize = 5;

mod weapon;
mod player;
mod bouncer;
mod avoider;
mod turret;
mod wall;
mod maze;

pub use self::weapon::{create_light_ray, create_weapon};
pub use self::player::create_player;
pub use self::bouncer::create_bouncer;
pub use self::avoider::create_avoider;
pub use self::turret::create_turret;
pub use self::wall::{create_wall_side, create_floor_ceil};
pub use self::maze::create_maze_walls;
