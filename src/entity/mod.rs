pub const WALL_GROUP: usize = 1; // Both vertical walls and ceil and floor
pub const FLOOR_CEIL_GROUP: usize = 2;
pub const ALIVE_GROUP: usize = 3;
pub const MONSTER_GROUP: usize = 4;
pub const TURRET_GROUP: usize = 5;
pub const PLAYER_GROUP: usize = 6;
pub const PLAYER_LASER_GROUP: usize = 7;
pub const ATTRACTED_VISION_GROUP: usize = 8;

mod depth_ball;
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
mod motionless;
mod attracted;

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
pub use self::depth_ball::*;
pub use self::motionless::*;
pub use self::attracted::*;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum EntityConf {
    Attracted { eraser: bool },
    Avoider { eraser: bool },
    Bouncer { eraser: bool },
    MotionLess { eraser: bool },
    Turret,
    Generator {
        generated_entity: ::component::GeneratedEntity,
        salvo: usize,
        time_between_salvo_ms: usize,
        eraser_probability_percent: usize,
    }
}

impl EntityConf {
    /// Used to position differently turrets from other entities
    pub fn is_turret_like(&self) -> bool {
        use self::EntityConf::*;
        match *self {
            Turret => true,
            _ => false,
        }
    }

    pub fn create(&self, pos: ::na::Vector3<f32>, world: &mut ::specs::World) {
        use self::EntityConf::*;
        match *self {
            Attracted { eraser } => create_attracted_w(pos, eraser, world),
            Avoider { eraser } => create_avoider_w(pos, eraser, world),
            Bouncer { eraser } => create_bouncer_w(pos, eraser, world),
            Turret => create_turret_w(pos, world),
            MotionLess { eraser } => create_motionless_w(pos, eraser, world),
            Generator {
                generated_entity,
                salvo,
                time_between_salvo_ms,
                eraser_probability_percent,
            } => create_generator(
                pos,
                generated_entity,
                salvo,
                time_between_salvo_ms as f32 / 1000.0,
                eraser_probability_percent as f32 / 100.0,
                &mut world.write_resource(),
                &world.read_resource(),
            ),
        }
    }
}
