use vulkano::command_buffer::AutoCommandBuffer;

pub use graphics::Data as Graphics;
pub use imgui::ImGui;
pub use maze::Maze;

pub type PhysicWorld = ::nphysics::world::World<f32>;
pub type WinitEvents = Vec<::winit::Event>;

pub struct Config {
    pub fps: u32,
    pub dt: f32,
    pub mouse_sensibility: f32,
}

// TODO: read from config file
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

pub struct Rendering {
    pub image_num: Option<usize>,
    pub command_buffer: Option<AutoCommandBuffer>,
    pub second_command_buffer: Option<AutoCommandBuffer>,
    pub size_points: Option<(u32, u32)>,
    pub size_pixels: Option<(u32, u32)>,
}

impl Rendering {
    pub fn new() -> Self {
        Rendering {
            image_num: None,
            command_buffer: None,
            second_command_buffer: None,
            size_points: None,
            size_pixels: None,
        }
    }
}
