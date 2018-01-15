use vulkano::command_buffer::AutoCommandBuffer;

pub use graphics::Data as Graphics;
pub use std::time::Duration;
pub use std::collections::HashMap;

pub type PhysicWorld = ::nphysics::world::World<f32>;
pub struct Events(pub Vec<::winit::Event>);
pub type Benchmarks = Vec<::util::Benchmark>;

pub type ImGuiOption = Option<::imgui::ImGui>;

pub struct FpsCounter(pub usize);

pub struct Save {
    pub mouse_sensibility: f32,
}

impl Save {
    pub fn new() -> Self {
        // TODO: write save if none and load from save
        Save {
            mouse_sensibility: ::CONFIG.mouse_sensibility,
        }
    }

    // pub fn load() -> Self {
    //     let file = File::open(SAVE_FILENAME).unwrap();
    //     ::ron::de::from_reader(file).unwrap()
    // }

    // pub fn save(&self) {
    //     let string = ::ron::ser::to_string(&self).unwrap();
    //     let mut file = File::open(SAVE_FILENAME).unwrap();
    //     file.write_all(string.as_bytes()).unwrap();
    // }
}

pub struct UpdateTime(pub f32);

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
    pub fn find_path(
        &self,
        pos: ::na::Vector3<f32>,
        goal: ::na::Vector3<f32>,
    ) -> Option<Vec<::na::Vector3<f32>>> {
        match *self {
            Maze::Maze2D(ref maze) => maze.find_path(pos, goal),
            Maze::Maze3D(ref maze) => maze.find_path(pos, goal),
        }
    }

    pub fn is_3d(&self) -> bool {
        match *self {
            Maze::Maze2D(_) => false,
            Maze::Maze3D(_) => true,
        }
    }
}

pub struct State {
    pub pause: bool,
    pub mouse_sensibility: f32,
    pub play_button: bool,
    pub reset_button: bool,
    pub quit_button: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            pause: true,
            mouse_sensibility: 0.001,
            play_button: false,
            reset_button: false,
            quit_button: false,
        }
    }

    pub fn build_ui(&mut self, ui: &::imgui::Ui) {
        let (width, height) = ui.imgui().display_size();
        if self.pause {
            ui.window(im_str!("Menu"))
                .collapsible(false)
                .position((width/4.0, height/4.0), ::imgui::ImGuiCond::FirstUseEver)
                .size((width/2.0, height/2.0), ::imgui::ImGuiCond::FirstUseEver)
                .resizable(true)
                .movable(true)
                .build(|| {
                    self.play_button = ui.small_button(im_str!("Play"));
                    ui.tree_node(im_str!("Levels"))
                        .build(|| {
                            for _ in 0..30 {
                                ui.small_button(im_str!("Level 1"));
                                ui.same_line(width/8.0);
                                ui.text(im_str!("TOTOTO"));
                            }
                        });
                    ui.tree_node(im_str!("Settings"))
                        .build(|| {
                            ui.input_float(im_str!("Mouse sensibility"), &mut self.mouse_sensibility)
                                .build();
                            self.reset_button = ui.small_button(im_str!("Reset"));
                        });
                    ui.tree_node(im_str!("Credits"))
                        .build(|| {
                            ui.text_wrapped(im_str!("Guillaume Thiolliere
TODO: others"));
                        });
                    self.quit_button = ui.small_button(im_str!("Quit"));
                });
        }
    }
}
