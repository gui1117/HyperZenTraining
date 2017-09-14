pub use ::graphics::Data as Graphics;

pub type WinitEvents = Vec<::winit::Event>;

pub struct Config {
    pub fps: u32,
    pub dt: f32,
    pub mouse_sensibility: f32,
}

// TODO read from config file
impl Default for Config {
    fn default() -> Self {
        let fps = 60;
        Config {
            fps,
            dt: 1.0 / fps as f32,
            mouse_sensibility: 1000.0,
        }
    }
}

// TODO change to aim that is assigned to player
pub struct Control {
    pub pointer: [f32; 2],
}

impl Control {
    pub fn new() -> Self {
        Control {
            pointer: [0.0, 0.0],
        }
    }
}

pub struct Rendering {
    pub image_num: Option<usize>,
    pub command_buffer: Option<::vulkano::command_buffer::AutoCommandBuffer>,
    pub second_command_buffer: Option<::vulkano::command_buffer::AutoCommandBuffer>,
}

impl Rendering {
    pub fn new() -> Self {
        Rendering {
            image_num: None,
            command_buffer: None,
            second_command_buffer: None,
        }
    }
}

pub struct PhysicWorld(pub ::nphysics::world::World<f32>);
unsafe impl Send for PhysicWorld {}
unsafe impl Sync for PhysicWorld {}

impl PhysicWorld {
    pub fn new() -> Self {
        let mut world = ::nphysics::world::World::new();
        PhysicWorld(world)
    }
}

