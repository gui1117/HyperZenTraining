pub const WALL_GROUP: usize = 1; // Both vertical walls and ceil and floor
pub const FLOOR_CEIL_GROUP: usize = 2;
pub const ALIVE_GROUP: usize = 3;
pub const MONSTER_GROUP: usize = 4;
pub const TURRET_GROUP: usize = 5;
pub const PLAYER_GROUP: usize = 6;

mod generator;
mod teleport;
mod weapon;
mod player;
mod bouncer;
mod avoider;
mod turret;
mod wall;
mod maze_2d;
mod maze_3d;

pub use self::generator::*;
pub use self::teleport::*;
pub use self::weapon::*;
pub use self::player::*;
pub use self::bouncer::*;
pub use self::avoider::*;
pub use self::turret::*;
pub use self::wall::*;
pub use self::maze_2d::*;
pub use self::maze_3d::*;
