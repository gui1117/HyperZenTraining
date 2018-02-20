use vulkano::command_buffer::AutoCommandBuffer;
use app_dirs::{AppInfo, app_root, AppDataType};

pub use graphics::Data as Graphics;
use std::io::Write;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use std::collections::HashMap;
use std::fmt;
use util::Direction;
pub use audio::Audio;
use show_message::OkOrShow;

pub type PhysicWorld = ::nphysics::world::World<f32>;
pub struct Events(pub Vec<::winit::Event>);
pub type Benchmarks = Vec<::util::Benchmark>;

pub struct Activated(pub bool);

pub type ImGuiOption = Option<::imgui::ImGui>;

pub struct FpsCounter(pub usize);

#[derive(Deserialize, Serialize)]
pub struct Save {
    mouse_sensibility: f32,
    scores: HashMap<usize, Score>,
    input_settings: InputSettings,
}

#[derive(Deserialize, Serialize)]
pub struct InputSettings {
    shoot: PossibleInput,
    forward: PossibleInput,
    backward: PossibleInput,
    left: PossibleInput,
    right: PossibleInput,
}

impl InputSettings {
    pub fn default() -> Self {
        InputSettings {
            shoot: PossibleInput::MouseButton(::winit::MouseButton::Left),
            forward: PossibleInput::VirtualKeyCode(::winit::VirtualKeyCode::W),
            backward: PossibleInput::VirtualKeyCode(::winit::VirtualKeyCode::S),
            left: PossibleInput::VirtualKeyCode(::winit::VirtualKeyCode::A),
            right: PossibleInput::VirtualKeyCode(::winit::VirtualKeyCode::D),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PossibleInput {
    #[serde(with = "::util::VirtualKeyCodeDef")]
    VirtualKeyCode(::winit::VirtualKeyCode),
    #[serde(with = "::util::MouseButtonDef")]
    MouseButton(::winit::MouseButton),
}

impl fmt::Display for PossibleInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PossibleInput::VirtualKeyCode(code) => {
                write!(f, "key {:?}", code)
            },
            PossibleInput::MouseButton(bouton) => {
                write!(f, "mouse {:?}", bouton)
            },
        }
    }
}

#[derive(Clone, Copy)]
pub enum Input {
    Shoot,
    Direction(Direction),
}

#[derive(Deserialize, Serialize)]
pub struct Score {
    pub bests: Vec<Duration>,
    pub lasts: Vec<Duration>,
}

impl Score {
    fn new() -> Self {
        Score {
            bests: vec![],
            lasts: vec![],
        }
    }
    fn insert(&mut self, duration: Duration) {
        self.bests.push(duration);
        self.bests.sort();
        self.bests.truncate(10);

        self.lasts.insert(0, duration);
        self.lasts.truncate(10);
    }
}

const APP_INFO: AppInfo = AppInfo { name: "pepe", author: "thiolliere" };
const FILENAME: &str = "save.ron";

lazy_static! {
    static ref SAVE_PATH: PathBuf = {
        let mut path = app_root(AppDataType::UserConfig, &APP_INFO).unwrap();
        path.push(FILENAME);
        path
    };
}

impl Save {
    pub fn set_input(&mut self, input: Input, set: PossibleInput) {
        match input {
            Input::Shoot => self.input_settings.shoot = set,
            Input::Direction(Direction::Forward) => self.input_settings.forward = set,
            Input::Direction(Direction::Backward) => self.input_settings.backward = set,
            Input::Direction(Direction::Left) => self.input_settings.left = set,
            Input::Direction(Direction::Right) => self.input_settings.right = set,
        }
        self.save();
    }

    pub fn input(&self, input: Input) -> PossibleInput {
        match input {
            Input::Shoot => self.input_settings.shoot.clone(),
            Input::Direction(Direction::Forward) => self.input_settings.forward.clone(),
            Input::Direction(Direction::Backward) => self.input_settings.backward.clone(),
            Input::Direction(Direction::Left) => self.input_settings.left.clone(),
            Input::Direction(Direction::Right) => self.input_settings.right.clone(),
        }
    }

    pub fn new() -> Self {
        File::open(SAVE_PATH.as_path()).ok()
            .and_then(|file| ::ron::de::from_reader(file).ok())
            .unwrap_or(Save {
                mouse_sensibility: ::CONFIG.mouse_sensibility,
                scores: HashMap::new(),
                input_settings: InputSettings::default()
            })
    }

    pub fn reset_input_settings(&mut self) {
        self.input_settings = InputSettings::default();
        self.save();
    }

    pub fn convert_keycode_input(&self, keycode: ::winit::VirtualKeyCode) -> Vec<Input> {
        let mut input = vec![];
        if let PossibleInput::VirtualKeyCode(c) = self.input_settings.shoot {
            if keycode == c { input.push(Input::Shoot) }
        }
        if let PossibleInput::VirtualKeyCode(c) = self.input_settings.forward {
            if keycode == c { input.push(Input::Direction(Direction::Forward)) }
        }
        if let PossibleInput::VirtualKeyCode(c) = self.input_settings.backward {
            if keycode == c { input.push(Input::Direction(Direction::Backward)) }
        }
        if let PossibleInput::VirtualKeyCode(c) = self.input_settings.left {
            if keycode == c { input.push(Input::Direction(Direction::Left)) }
        }
        if let PossibleInput::VirtualKeyCode(c) = self.input_settings.right {
            if keycode == c { input.push(Input::Direction(Direction::Right)) }
        }
        input
    }

    pub fn convert_mouse_button_input(&self, button: ::winit::MouseButton) -> Vec<Input> {
        let mut input = vec![];
        if let PossibleInput::MouseButton(b) = self.input_settings.shoot {
            if button == b { input.push(Input::Shoot) }
        }
        if let PossibleInput::MouseButton(b) = self.input_settings.forward {
            if button == b { input.push(Input::Direction(Direction::Forward)) }
        }
        if let PossibleInput::MouseButton(b) = self.input_settings.backward {
            if button == b { input.push(Input::Direction(Direction::Backward)) }
        }
        if let PossibleInput::MouseButton(b) = self.input_settings.left {
            if button == b { input.push(Input::Direction(Direction::Left)) }
        }
        if let PossibleInput::MouseButton(b) = self.input_settings.right {
            if button == b { input.push(Input::Direction(Direction::Right)) }
        }
        input
   }

    #[inline]
    pub fn mouse_sensibility(&self) -> f32 {
        self.mouse_sensibility
    }

    pub fn insert_score(&mut self, level: usize, score: Duration) {
        self.scores.entry(level).or_insert(Score::new()).insert(score);
        self.save();
    }

    pub fn score(&self, level: usize) -> Option<&Score> {
        self.scores.get(&level)
    }

    /// Do nothing if sensibility hasn't changed
    pub fn set_mouse_sensibility_if_changed(&mut self, mouse_sensibility: f32) {
        if self.mouse_sensibility != mouse_sensibility {
            self.mouse_sensibility = mouse_sensibility;
            self.save();
        }
    }

    pub fn save(&self) {
        let string = ::ron::ser::to_string(&self).unwrap();
        let mut file = File::create(SAVE_PATH.as_path())
            .ok_or_show(|e| format!("Failed to create save file at {}: {}", SAVE_PATH.to_string_lossy(), e));
        file.write_all(string.as_bytes())
            .ok_or_show(|e| format!("Failed to write to save file {}: {}", SAVE_PATH.to_string_lossy(), e));
    }
}

pub struct UpdateTime(pub f32);

pub struct GameDuration(pub Duration);

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

pub struct LevelActions(pub Vec<LevelAction>);

#[derive(Clone)]
pub enum LevelAction {
    Next,
    Reset,
    ReturnHall,
    Level(usize),
}

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
}

pub enum MenuStateState {
    Pause,
    Input(Input),
    Game,
}

pub struct MenuState {
    pub state: MenuStateState,
    pub mouse_sensibility_input: f32,
    pub continue_button: bool,
    pub reset_button: bool,
    pub return_hall_button: bool,
    pub set_shoot_button: bool,
    pub set_forward_button: bool,
    pub set_backward_button: bool,
    pub set_left_button: bool,
    pub set_right_button: bool,
    pub quit_button: bool,
    pub levels_button: [bool; 16],
}

impl MenuState {
    pub fn paused(&self) -> bool {
        match self.state {
            MenuStateState::Input(_) => true,
            MenuStateState::Pause => true,
            MenuStateState::Game => false,
        }
    }

    pub fn new(save: &Save) -> Self {
        MenuState {
            state: MenuStateState::Game,
            mouse_sensibility_input: save.mouse_sensibility(),
            continue_button: false,
            reset_button: false,
            set_shoot_button: false,
            set_forward_button: false,
            set_backward_button: false,
            set_left_button: false,
            set_right_button: false,
            return_hall_button: false,
            quit_button: false,
            levels_button: [false; 16],
        }
    }

    pub fn build_ui(&mut self, ui: &::imgui::Ui, save: &Save) {
        let (width, height) = ui.imgui().display_size();
        let button_size = (76.0, 30.0);

        match self.state {
            MenuStateState::Pause | MenuStateState::Input(_) => {
                let inputs = if let MenuStateState::Pause = self.state {
                    true
                } else {
                    false
                };

                ui.window(im_str!("Pause"))
                    .collapsible(false)
                    .inputs(inputs)
                    .size((::CONFIG.menu_width, ::CONFIG.menu_height), ::imgui::ImGuiCond::Always)
                    .position((width/2.0-::CONFIG.menu_width/2.0, height/2.0-::CONFIG.menu_height/2.0), ::imgui::ImGuiCond::Always)
                    .resizable(false)
                    .movable(false)
                    .build(|| {
                        self.continue_button = ui.button(im_str!("Continue"), button_size);
                        ui.separator();
                        self.return_hall_button = ui.button(im_str!("Return to hall"), button_size);
                        ui.separator();
                        ui.text("Settings :");
                        ui.separator();
                        ui.input_float(im_str!("Mouse sensibility"), &mut self.mouse_sensibility_input).build();

                        ui.text(format!("Shoot: {}", save.input(Input::Shoot)));
                        ui.same_line(0.0);
                        self.set_shoot_button = ui.small_button(im_str!("Set shoot"));

                        ui.text(format!("Forward: {}", save.input(Input::Direction(Direction::Forward))));
                        ui.same_line(0.0);
                        self.set_forward_button = ui.small_button(im_str!("Set forward"));

                        ui.text(format!("Left: {}", save.input(Input::Direction(Direction::Left))));
                        ui.same_line(0.0);
                        self.set_left_button = ui.small_button(im_str!("Set left"));

                        ui.text(format!("Backward: {}", save.input(Input::Direction(Direction::Backward))));
                        ui.same_line(0.0);
                        self.set_backward_button = ui.small_button(im_str!("Set backward"));

                        ui.text(format!("Right: {}", save.input(Input::Direction(Direction::Right))));
                        ui.same_line(0.0);
                        self.set_right_button = ui.small_button(im_str!("Set right"));

                        self.reset_button = ui.button(im_str!("Reset"), button_size);
                        ui.separator();
                        self.quit_button = ui.button(im_str!("Quit"), button_size);
                    });
            },
            _ => (),
        }

        match self.state {
            MenuStateState::Input(_) => {
                ui.window(im_str!("Input"))
                    .collapsible(false)
                    .size((::CONFIG.menu_width/2.0, ::CONFIG.menu_height/2.0), ::imgui::ImGuiCond::Always)
                    .position((width/2.0-::CONFIG.menu_width/4.0, height/2.0-::CONFIG.menu_height/4.0), ::imgui::ImGuiCond::Always)
                    .resizable(false)
                    .movable(false)
                    .build(|| {
                        ui.text("Set input or escape");
                    });
            },
            _ => (),
        }
    }
}
