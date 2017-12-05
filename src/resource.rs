use vulkano::command_buffer::AutoCommandBuffer;
use util::ConvCoord;

pub use graphics::Data as Graphics;
pub use imgui::ImGui;
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

pub struct DepthCoef(pub f32);

pub struct PlayerControl {
    pub directions: Vec<::util::Direction>,
    pub pointer: [f32; 2],
}

impl PlayerControl {
    pub fn new() -> Self {
        PlayerControl {
            directions: vec![],
            pointer: [0.0, 0.0],
        }
    }
}

// TODO: change into an enum that can be Retry, Next, None
pub struct EndLevel(pub bool);

pub enum Maze {
    Maze2D(::maze::Maze<::na::U2>),
    Maze3D(::maze::Maze<::na::U3>),
}

impl Maze {
    pub fn find_path(&self, pos: ::na::Vector3<f32>, goal: ::na::Vector3<f32>) -> Option<Vec<::na::Vector3<f32>>> {
        match *self {
            Maze::Maze2D(ref maze) => {
                let path = maze.find_path(pos.conv_2isize(), goal.conv_2isize());
                path.map(|p| p.iter().map(|c| c.conv_3f32()).collect::<Vec<_>>())
            }
            Maze::Maze3D(ref maze) => {
                let path = maze.find_path(pos.conv_3isize(), goal.conv_3isize());
                path.map(|p| p.iter().map(|c| c.conv_3f32()).collect::<Vec<_>>())
            }
        }
    }

    pub fn is_3d(&self) -> bool {
        match *self {
            Maze::Maze2D(_) => false,
            Maze::Maze3D(_) => true,
        }
    }
}
