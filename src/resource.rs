use vulkano::command_buffer::AutoCommandBuffer;
use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;
use app_dirs2::{AppInfo, app_root, AppDataType};

use imgui::ImString;
pub use graphics::Graphics;
use std::io::Write;
use std::sync::Arc;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::ffi::OsStr;
use std::io::Read;
use std::io::Cursor;
use util::Direction;
pub use audio::Audio;
use show_message::UnwrapOrShow;
use winit::MouseButton;

pub type PhysicWorld = ::nphysics::world::World<f32>;
pub struct Events(pub Vec<::winit::Event>);
pub type Benchmarks = Vec<::util::Benchmark>;
pub type VulkanInstance = Arc<Instance>;

pub struct Activated(pub bool);

pub type ImGuiOption = Option<::imgui::ImGui>;

pub struct FpsCounter(pub usize);

#[derive(Deserialize, Serialize)]
pub struct Save {
    mouse_sensibility: f32,
    scores: HashMap<usize, Score>,
    input_settings: InputSettings,
    fullscreen: bool,
    vulkan_device_uuid: Option<[u8; 16]>,
    field_of_view: f32,
    effect_volume: f32,
    music_volume: f32,
    custom_level_conf: CustomLevelConf,
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

impl PossibleInput {
    fn text(&self, text: &Text) -> String {
        match *self {
            PossibleInput::VirtualKeyCode(code) => {
                format!("{} {:?}", text.key, code)
            },
            PossibleInput::MouseButton(MouseButton::Other(n)) => {
                format!("{} {:?}", text.mouse_other, n)
            },
            PossibleInput::MouseButton(MouseButton::Middle) => {
                format!("{}", text.mouse_middle)
            },
            PossibleInput::MouseButton(MouseButton::Left) => {
                format!("{}", text.mouse_left)
            },
            PossibleInput::MouseButton(MouseButton::Right) => {
                format!("{}", text.mouse_right)
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

const APP_INFO: AppInfo = AppInfo { name: "HyperZen Training", author: "thiolliere" };
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
                input_settings: InputSettings::default(),
                fullscreen: true,
                vulkan_device_uuid: None,
                effect_volume: 1.0,
                music_volume: 1.0,
                field_of_view: ::CONFIG.field_of_view,
                custom_level_conf: CustomLevelConf::default(),
            })
    }

    pub fn set_custom_level_conf_lazy(&mut self, custom_level_conf: CustomLevelConf) {
        if self.custom_level_conf != custom_level_conf {
            self.custom_level_conf = custom_level_conf;
            self.save();
        }
    }

    pub fn custom_level_conf(&self) -> CustomLevelConf {
        self.custom_level_conf.clone()
    }

    pub fn set_field_of_view_lazy(&mut self, field_of_view: f32) {
        if self.field_of_view != field_of_view {
            self.field_of_view = field_of_view;
            self.save();
        }
    }

    pub fn field_of_view(&self) -> f32 {
        self.field_of_view
    }

    pub fn set_effect_volume_lazy(&mut self, volume: f32) {
        if self.effect_volume != volume {
            self.effect_volume = volume;
            self.save();
        }
    }

    pub fn effect_volume(&self) -> f32 {
        self.effect_volume
    }

    pub fn set_music_volume_lazy(&mut self, volume: f32) {
        if self.music_volume != volume {
            self.music_volume = volume;
            self.save();
        }
    }

    pub fn music_volume(&self) -> f32 {
        self.music_volume
    }

    /// Return if changed
    pub fn set_vulkan_device_uuid_lazy(&mut self, uuid: &[u8; 16]) -> bool {
        if self.vulkan_device_uuid.map(|saved_uuid| *uuid != saved_uuid).unwrap_or(true) {
            self.vulkan_device_uuid = Some(uuid.clone());
            self.save();
            true
        } else {
            false
        }
    }

    pub fn vulkan_device_uuid(&self) -> &Option<[u8; 16]> {
        &self.vulkan_device_uuid
    }

    pub fn reset_controls(&mut self) {
        self.input_settings = InputSettings::default();
        self.field_of_view = ::CONFIG.field_of_view;
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
    pub fn set_mouse_sensibility_lazy(&mut self, mouse_sensibility: f32) {
        if self.mouse_sensibility != mouse_sensibility {
            self.mouse_sensibility = mouse_sensibility;
            self.save();
        }
    }

    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
        self.save();
    }

    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn save(&self) {
        let string = ::ron::ser::to_string(&self).unwrap();
        let mut file = File::create(SAVE_PATH.as_path())
            .unwrap_or_else_show(|e| format!("Failed to create save file at {}: {}", SAVE_PATH.display(), e));
        file.write_all(string.as_bytes())
            .unwrap_or_else_show(|e| format!("Failed to write to save file {}: {}", SAVE_PATH.display(), e));
    }
}

pub struct UpdateTime(pub f32);

pub struct GameDuration(pub Duration);

pub struct Rendering {
    pub image_num: Option<usize>,
    pub command_buffer: Option<AutoCommandBuffer>,
    pub second_command_buffer: Option<AutoCommandBuffer>,
    pub size: Option<(u32, u32)>,
}

impl Rendering {
    pub fn new() -> Self {
        Rendering {
            image_num: None,
            command_buffer: None,
            second_command_buffer: None,
            size: None,
        }
    }
}

pub struct ErasedStatus {
    pub need_buffer_clear: bool,
    pub amount: f32,
}

impl ErasedStatus {
    pub fn new() -> Self {
        ErasedStatus {
            need_buffer_clear: false,
            amount: 0.0,
        }
    }

    pub fn clear(&mut self) {
        self.need_buffer_clear = true;
        self.amount = 0.0;
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
    Custom,
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
    Help,
    Restart,
    CreateCustom,
}

#[derive(PartialEq, Deserialize, Serialize, Clone)]
pub struct CustomLevelConf {
    pub maze_size: i32,
    pub x_shift: bool,
    pub y_shift: bool,
    pub percent: f32,
    pub motion_less: i32,
    pub motion_less_eraser: i32,
    pub attracted: i32,
    pub attracted_eraser: i32,
    pub bouncer: i32,
    pub bouncer_eraser: i32,
    pub avoider: i32,
    pub avoider_eraser: i32,
    pub turret: i32,
}

impl CustomLevelConf {
    pub fn default() -> Self {
        CustomLevelConf {
            maze_size: 10,
            x_shift: false,
            y_shift: false,
            percent: 5.0,
            motion_less: 3,
            motion_less_eraser: 3,
            attracted: 0,
            attracted_eraser: 0,
            bouncer: 0,
            bouncer_eraser: 0,
            avoider: 0,
            avoider_eraser: 0,
            turret: 0,
        }
    }
}

pub struct MenuState {
    pub state: MenuStateState,
    pub mouse_sensibility_input: f32,
    pub continue_button: bool,
    pub reset_button: bool,
    pub return_hall_button: bool,
    pub set_shoot_button: bool,
    pub fullscreen_checkbox: bool,
    pub set_forward_button: bool,
    pub set_backward_button: bool,
    pub set_left_button: bool,
    pub restart_now_button: bool,
    pub restart_later_button: bool,
    pub help_ok_button: bool,
    pub help_button: bool,
    pub set_right_button: bool,
    pub quit_button: bool,
    pub levels_button: [bool; 16],
    pub vulkan_device: [u8; 16],
    pub effect_volume_slider: f32,
    pub music_volume_slider: f32,
    pub field_of_view_slider: f32,

    pub create_custom_button: bool,
    pub custom_return_button: bool,
    pub custom_play_button: bool,
    pub custom_level_conf: CustomLevelConf,
}

impl MenuState {
    pub fn paused(&self) -> bool {
        match self.state {
            MenuStateState::Input(_) => true,
            MenuStateState::Pause => true,
            MenuStateState::Restart => true,
            MenuStateState::CreateCustom => true,
            MenuStateState::Game => false,
            MenuStateState::Help => true,
        }
    }

    pub fn new(save: &Save) -> Self {
        MenuState {
            state: MenuStateState::Game,
            mouse_sensibility_input: save.mouse_sensibility(),
            fullscreen_checkbox: save.fullscreen(),
            vulkan_device: save.vulkan_device_uuid().expect("Cannot create menu without saved vulkan device"),
            continue_button: false,
            reset_button: false,
            set_shoot_button: false,
            restart_now_button: false,
            restart_later_button: false,
            help_ok_button: false,
            help_button: false,
            set_forward_button: false,
            set_backward_button: false,
            set_left_button: false,
            field_of_view_slider: save.field_of_view(),
            set_right_button: false,
            return_hall_button: false,
            quit_button: false,
            levels_button: [false; 16],
            music_volume_slider: save.effect_volume(),
            effect_volume_slider: save.music_volume(),

            create_custom_button: false,
            custom_return_button: false,
            custom_play_button: false,
            custom_level_conf: save.custom_level_conf(),
        }
    }

    pub fn build_ui(&mut self, ui: &::imgui::Ui, save: &Save, vulkan_instance: &VulkanInstance, text: &Text, help: &String) {
        let (width, height) = ui.imgui().display_size();
        let button_size = (::CONFIG.menu_width - 16.0, 30.0);
        let small_button_size = (80.0, 20.0);
        let medium_button_size = (::CONFIG.menu_width/3.0-12.0, 30.0);
        let medium_button_size_2 = (::CONFIG.menu_width*2.0/3.0 - 16.0, 30.0);

        match self.state {
            MenuStateState::Pause | MenuStateState::Input(_) | MenuStateState::Restart | MenuStateState::Help => {
                let inputs = if let MenuStateState::Pause = self.state {
                    true
                } else {
                    false
                };

                ui.window(&ImString::new(text.pause.clone()))
                    .collapsible(false)
                    .inputs(inputs)
                    .size((::CONFIG.menu_width, ::CONFIG.menu_height), ::imgui::ImGuiCond::Always)
                    .position((width/2.0-::CONFIG.menu_width/2.0, height/2.0-::CONFIG.menu_height/2.0), ::imgui::ImGuiCond::Always)
                    .resizable(false)
                    .movable(false)
                    .build(|| {
                        self.continue_button = ui.button(&ImString::new(text.continue_.clone()), button_size);
                        self.return_hall_button = ui.button(&ImString::new(text.return_to_hall.clone()), button_size);
                        self.create_custom_button = ui.button(&ImString::new(text.create_custom_level.clone()), button_size);
                        self.help_button = ui.button(&ImString::new(text.help.clone()), button_size);
                        self.quit_button = ui.button(&ImString::new(text.quit.clone()), button_size);
                        ui.separator();
                        ui.text(&ImString::new(text.audio.clone()));
                        ui.slider_float(&ImString::new(text.music_volume.clone()), &mut self.effect_volume_slider, 0.0, 1.0).build();
                        ui.slider_float(&ImString::new(text.effect_volume.clone()), &mut self.music_volume_slider, 0.0, 1.0).build();

                        ui.separator();
                        ui.text(&ImString::new(text.video.clone()));

                        self.fullscreen_checkbox = ui.checkbox(&ImString::new(text.fullscreen.clone()), &mut save.fullscreen());

                        for device in PhysicalDevice::enumerate(vulkan_instance) {
                            let cond = save.vulkan_device_uuid()
                                .map(|uuid| uuid == *device.uuid())
                                .unwrap();

                            let mut name = device.name().as_bytes().to_vec();
                            name.push(b'\0');

                            let name = unsafe {
                                ::imgui::ImStr::from_utf8_with_nul_unchecked(&name)
                            };

                            if ui.radio_button_bool(name, cond) {
                                self.vulkan_device = *device.uuid();
                            }
                        }

                        ui.separator();
                        ui.text(&ImString::new(text.controls.clone()));
                        ui.same_line(0.0);
                        self.reset_button = ui.button(&ImString::new(text.reset.clone()), small_button_size);

                        ui.slider_float(&ImString::new(text.field_of_view.clone()), &mut self.field_of_view_slider, 0.1, 2.0).build();
                        ui.input_float(&ImString::new(text.mouse_sensibility.clone()), &mut self.mouse_sensibility_input).build();

                        self.set_shoot_button = ui.button(&ImString::new(text.shoot.clone()), small_button_size);
                        ui.same_line(0.0);
                        ui.text(format!("[{}]", save.input(Input::Shoot).text(text)));

                        self.set_forward_button = ui.button(&ImString::new(text.forward.clone()), small_button_size);
                        ui.same_line(0.0);
                        ui.text(format!("[{}]", save.input(Input::Direction(Direction::Forward)).text(text)));

                        self.set_left_button = ui.button(&ImString::new(text.left.clone()), small_button_size);
                        ui.same_line(0.0);
                        ui.text(format!("[{}]", save.input(Input::Direction(Direction::Left)).text(text)));

                        self.set_backward_button = ui.button(&ImString::new(text.backward.clone()), small_button_size);
                        ui.same_line(0.0);
                        ui.text(format!("[{}]", save.input(Input::Direction(Direction::Backward)).text(text)));

                        self.set_right_button = ui.button(&ImString::new(text.right.clone()), small_button_size);
                        ui.same_line(0.0);
                        ui.text(format!("[{}]", save.input(Input::Direction(Direction::Right)).text(text)));

                        ui.separator();
                        ui.text(&ImString::new(text.credits.clone()));
                        ui.text("    Guillaume Thiolliere  http://thiolliere.org");
                    });
            },
            MenuStateState::CreateCustom => {
                ui.window(&ImString::new(text.custom.clone()))
                    .collapsible(false)
                    .size((::CONFIG.menu_width, ::CONFIG.menu_height), ::imgui::ImGuiCond::Always)
                    .position((width/2.0-::CONFIG.menu_width/2.0, height/2.0-::CONFIG.menu_height/2.0), ::imgui::ImGuiCond::Always)
                    .resizable(false)
                    .movable(false)
                    .build(|| {
                        self.custom_play_button = ui.button(&ImString::new(text.play.clone()), button_size);
                        self.custom_return_button = ui.button(&ImString::new(text.return_.clone()), button_size);
                        ui.separator();
                        ui.text(&ImString::new(text.configuration.clone()));

                        ui.slider_int(&ImString::new(text.size.clone()), &mut self.custom_level_conf.maze_size, 5, 30).build();
                        ui.checkbox(&ImString::new(text.x_shift.clone()), &mut self.custom_level_conf.x_shift);
                        ui.same_line(0.0);
                        ui.checkbox(&ImString::new(text.y_shift.clone()), &mut self.custom_level_conf.y_shift);

                        ui.slider_float(&ImString::new(text.filling.clone()), &mut self.custom_level_conf.percent, 0.0, 30.0).build();
                        ui.slider_int(&ImString::new(text.motionless.clone()), &mut self.custom_level_conf.motion_less, 0, 100).build();
                        ui.slider_int(&ImString::new(text.motionless_eraser.clone()), &mut self.custom_level_conf.motion_less_eraser, 0, 100).build();
                        ui.slider_int(&ImString::new(text.attracted.clone()), &mut self.custom_level_conf.attracted, 0, 100).build();
                        ui.slider_int(&ImString::new(text.attracted_eraser.clone()), &mut self.custom_level_conf.attracted_eraser, 0, 100).build();
                        ui.slider_int(&ImString::new(text.bouncer.clone()), &mut self.custom_level_conf.bouncer, 0, 100).build();
                        ui.slider_int(&ImString::new(text.bouncer_eraser.clone()), &mut self.custom_level_conf.bouncer_eraser, 0, 100).build();
                        ui.slider_int(&ImString::new(text.avoider.clone()), &mut self.custom_level_conf.avoider, 0, 100).build();
                        ui.slider_int(&ImString::new(text.avoider_eraser.clone()), &mut self.custom_level_conf.avoider_eraser, 0, 100).build();
                        ui.slider_int(&ImString::new(text.turret.clone()), &mut self.custom_level_conf.turret, 0, 100).build();
                    });
            }
            _ => (),
        }

        match self.state {
            MenuStateState::Input(_) => {
                ui.window(&ImString::new(text.input.clone()))
                    .collapsible(false)
                    .size((::CONFIG.menu_width/2.0, ::CONFIG.menu_height/2.0), ::imgui::ImGuiCond::Always)
                    .position((width/2.0-::CONFIG.menu_width/4.0, height/2.0-::CONFIG.menu_height/4.0), ::imgui::ImGuiCond::Always)
                    .resizable(false)
                    .movable(false)
                    .build(|| {
                        ui.text(&ImString::new(text.set_input_or_escape.clone()));
                    });
            },
            MenuStateState::Restart => {
                ui.window(&ImString::new(text.restart.clone()))
                    .collapsible(false)
                    .size((::CONFIG.menu_width/1.5, ::CONFIG.menu_height/2.0), ::imgui::ImGuiCond::Always)
                    .position((width/2.0-::CONFIG.menu_width/3.0, height/2.0-::CONFIG.menu_height/4.0), ::imgui::ImGuiCond::Always)
                    .resizable(false)
                    .movable(false)
                    .build(|| {
                        ui.text(&ImString::new(text.setting_needs_to_restart_the_game.clone()));
                        self.restart_now_button = ui.button(&ImString::new(text.restart_now.clone()), medium_button_size);
                        ui.same_line(0.0);
                        self.restart_later_button = ui.button(&ImString::new(text.restart_later.clone()), medium_button_size);
                    });
            }
            MenuStateState::Help=> {
                ui.window(&ImString::new(text.help.clone()))
                    .collapsible(false)
                    .size((::CONFIG.menu_width/1.5, ::CONFIG.menu_height/2.0), ::imgui::ImGuiCond::Always)
                    .position((width/2.0-::CONFIG.menu_width/3.0, height/2.0-::CONFIG.menu_height/4.0), ::imgui::ImGuiCond::Always)
                    .resizable(false)
                    .movable(false)
                    .build(|| {
                        ui.text(help);
                        self.help_ok_button = ui.button(&ImString::new(text.ok.clone()), medium_button_size_2);
                    });
            }
            _ => (),
        }
    }
}

pub struct Help(pub String);

#[derive(Deserialize)]
pub struct Text {
    pub pause: String,
    pub continue_: String,
    pub return_to_hall: String,
    pub create_custom_level: String,
    pub help: String,
    pub quit: String,
    pub audio: String,
    pub music_volume: String,
    pub effect_volume: String,
    pub video: String,
    pub fullscreen: String,
    pub controls: String,
    pub reset: String,
    pub field_of_view: String,
    pub mouse_sensibility: String,
    pub shoot: String,
    pub forward: String,
    pub backward: String,
    pub right: String,
    pub left: String,
    pub credits: String,
    pub custom: String,
    pub play: String,
    pub return_: String,
    pub configuration: String,
    pub size: String,
    pub x_shift: String,
    pub y_shift: String,
    pub filling: String,
    pub attracted_eraser: String,
    pub avoider_eraser: String,
    pub bouncer_eraser: String,
    pub motionless_eraser: String,
    pub turret: String,
    pub input: String,
    pub set_input_or_escape: String,
    pub restart: String,
    pub setting_needs_to_restart_the_game: String,
    pub restart_now: String,
    pub restart_later: String,
    pub ok: String,
    pub attracted: String,
    pub avoider: String,
    pub bouncer: String,
    pub motionless: String,
    pub go_to_portal: String,
    pub remains: String,
    pub mouse_middle: String,
    pub mouse_left: String,
    pub mouse_right: String,
    pub mouse_other: String,
    pub key: String,
    pub vulkan_error: String,
}

impl Text {
    pub fn load() -> Self {
        let mut texts: Vec<(String, _)> = if cfg!(feature = "packed") {
            vec![
                ("en-US".into(), Box::new(Cursor::new(include_str!("../assets/lang/en-US.ron"))) as Box<Read>),
            ]
        } else {
            let mut texts = vec![];
            let read_dir = fs::read_dir("assets/lang")
                .unwrap_or_else_show(|e| format!("Failed to open \"assets/lang\": {}", e));
            for entry in read_dir {
                let path = entry
                    .unwrap_or_else_show(|e| format!("Failed to open \"assets/lang\" entry: {}", e))
                    .path();
                if path.extension() == Some(OsStr::new("ron")) {
                    let lang = path.file_stem()
                        .unwrap_or_show(format!("Failed to get stem of {:?}", path))
                        .to_str()
                        .unwrap_or_show(format!("Failed to get stem of {:?}: Invalid UTF-8", path))
                        .into();
                    let mut file = File::open(&path)
                        .unwrap_or_else_show(|e| format!("Failed to open \"{}\": {}", path.to_string_lossy(), e));
                    texts.push((lang, Box::new(file) as Box<Read>));
                } else {
                    eprintln!("Invalid file: {}", path.to_string_lossy());
                }
            }
            texts
        };

        let requested = ::locale_config::Locale::user_default();
        let requested = requested.tags().map(|tag| tag.1).collect::<Vec<_>>();
        let available = texts.iter().map(|t| t.0.clone()).collect::<Vec<_>>();

        let supported = ::fluent_locale::negotiate_languages(
            &requested.iter().map(|tag| tag.as_ref()).collect::<Vec<_>>(),
            &available.iter().map(|t| t.as_str()).collect::<Vec<_>>(),
            Some("en-US"),
            &::fluent_locale::negotiate::NegotiationStrategy::Filtering
        );

        let file = texts.drain(..).find(|(lang, _)| lang == supported[0]).unwrap().1;
        ::ron::de::from_reader(file)
            .unwrap_or_else_show(|e| format!("Failed to parse lang file {}: {}", supported[0], e))
    }
}

#[test]
fn test_text() {
    for entry in fs::read_dir("assets/lang").unwrap() {
        let path = entry.unwrap().path();
        println!("{}", path.to_string_lossy());
        if path.extension() == Some(OsStr::new("ron")) {
            let mut file = File::open(&path).unwrap();
            let _: Text = ::ron::de::from_reader(file).unwrap();
        }
    }
}
