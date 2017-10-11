use vulkano::command_buffer::AutoCommandBuffer;

pub use graphics::Data as Graphics;
pub use imgui::ImGui;
pub use maze::Maze;
pub use config::Config;

pub type PhysicWorld = ::nphysics::world::World<f32>;
pub struct MenuEvents(pub Vec<::winit::Event>);
pub struct GameEvents(pub Vec<::winit::Event>);

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

pub struct DebugMode(pub bool);
