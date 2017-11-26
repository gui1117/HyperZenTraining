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
mod maze;

pub use self::generator::create_generator;
pub use self::teleport::{create_teleport, create_teleport_w};
pub use self::weapon::{create_light_ray, create_weapon};
pub use self::player::{create_player, create_player_w};
pub use self::bouncer::{create_bouncer, create_bouncer_w};
pub use self::avoider::{create_avoider, create_avoider_w};
pub use self::turret::{create_turret, create_turret_w};
pub use self::wall::{create_wall_side, create_floor_ceil};
pub use self::maze::{create_maze_walls, create_maze_walls_w};
