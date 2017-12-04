pub mod kill_all;
pub mod walkthrough;
pub mod walkthrough_3d;

#[derive(Serialize, Deserialize, Clone)]
pub enum Level {
    KillAll(kill_all::Conf),
    Walkthrough(walkthrough::Conf),
    Walkthrough3D(walkthrough_3d::Conf),
}

impl Level {
    pub fn create(&self, world: &mut ::specs::World) {
        match *self {
            Level::KillAll(ref conf) => kill_all::create(world, conf),
            Level::Walkthrough(ref conf) => walkthrough::create(world, conf),
            Level::Walkthrough3D(ref conf) => walkthrough_3d::create(world, conf),
        }
    }
}
