pub use ::ColWorld;
pub use ::graphics::Data as Graphics;

pub struct Control {
    pub directions: Vec<::util::Direction>,
    pub pointer: [f32; 2],
}

impl Control {
    pub fn new() -> Self {
        Control {
            directions: vec!(),
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
