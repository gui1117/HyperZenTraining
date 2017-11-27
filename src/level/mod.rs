pub mod kill_all;
pub mod walkthrough;

#[derive(Serialize, Deserialize, Clone)]
pub enum Level {
    KillAll(kill_all::Conf),
    Walkthrough(walkthrough::Conf),
}

impl Level {
    pub fn create(&self, world: &mut ::specs::World) {
        match *self {
            Level::KillAll(ref conf) => kill_all::create(world, conf),
            Level::Walkthrough(ref conf) => walkthrough::create(world, conf),
        }
    }
}
