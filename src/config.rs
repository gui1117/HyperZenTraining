use imgui::{ImGuiStyle, ImVec2, ImVec4};

use std::fs::File;
use std::io::Write;

const SAVE_FILENAME: &str = "config.ron";

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub style: ImGuiStyleSave,
    pub mouse_sensibility: f32,
    pub fps: u32,
    /// Not loaded
    pub debug_fps_counter: usize,
    pub eraser_time: f32,

    pub accumulated_impulse_solver_step: f32,
    pub correction_mode_a: f32,
    pub correction_mode_b: f32,
    pub correction_mode_c: f32,
    pub accumulated_impulse_solver_joint_corr_factor: f32,
    pub accumulated_impulse_solver_rest_eps: f32,
    pub accumulated_impulse_solver_num_first_order_iter: usize,
    pub accumulated_impulse_solver_num_second_order_iter: usize,

    pub avoider_size: f32,
    pub avoider_velocity: f32,
    pub avoider_time_to_reach_vmax: f32,
    pub avoider_ang_damping: f32,
    pub avoider_color: ::graphics::Color,
    pub avoider_avoid_norm: f32,

    pub bouncer_size: f32,
    pub bouncer_velocity: f32,
    pub bouncer_time_to_reach_vmax: f32,
    pub bouncer_ang_damping: f32,
    pub bouncer_color: ::graphics::Color,

    pub avoider_generator_salvo: usize,
    pub avoider_generator_eraser_probability: f32,
    pub avoider_generator_time_between_salvo: f32,
    pub bouncer_generator_salvo: usize,
    pub bouncer_generator_eraser_probability: f32,
    pub bouncer_generator_time_between_salvo: f32,

    pub player_height: f32,
    pub player_radius: f32,
    pub player_velocity: f32,
    pub player_time_to_reach_vmax: f32,
    pub player_ang_damping: f32,
    pub player_air_damping: f32,
    pub player_gravity: f32,
    pub player_hook_force: f32,

    pub teleport_dl: f32,

    pub laser_size: f32,
    pub laser_velocity: f32,
    pub laser_time_to_reach_vmax: f32,
    pub laser_ang_damping: f32,
    pub laser_amortization: f32,
    pub laser_color: ::graphics::Color,

    pub turret_size: f32,
    pub turret_velocity: f32,
    pub turret_time_to_reach_vmax: f32,
    pub turret_ang_damping: f32,
    pub turret_color: ::graphics::Color,

    pub wall_color: ::graphics::Color,
    pub floor_ceil_color: ::graphics::Color,

    pub weapon_reload_time: f32,
    pub weapon_bullet_nbr: usize,
    pub weapon_bullet_radius: f32,
    pub weapon_bullet_length: f32,
    pub weapon_bullet_x: f32,
    pub weapon_bullet_dx: f32,
    pub weapon_bullet_color: ::graphics::Color,
    pub weapon_six_color: ::graphics::Color,
    pub weapon_angle_color: ::graphics::Color,

    pub levels: Vec<::level::Level>,
}

impl Config {
    #[inline]
    pub fn dt(&self) -> f32 {
        1.0 / self.fps as f32
    }

    pub fn load() -> Self {
        let file = File::open(SAVE_FILENAME).unwrap();
        ::ron::de::from_reader(file).unwrap()
    }

    pub fn save(&self) {
        let string = ::ron::ser::to_string(&self).unwrap();
        let mut file = File::open(SAVE_FILENAME).unwrap();
        file.write_all(string.as_bytes()).unwrap();
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ImVec2")]
pub struct ImVec2Def {
    pub x: f32,
    pub y: f32,
}

impl From<ImVec2Def> for ImVec2 {
    fn from(def: ImVec2Def) -> Self {
        ImVec2 { x: def.x, y: def.y }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ImVec4")]
pub struct ImVec4Def {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<ImVec4Def> for ImVec4 {
    fn from(def: ImVec4Def) -> Self {
        ImVec4 {
            x: def.x,
            y: def.y,
            z: def.z,
            w: def.w,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImColorsSave {
    #[serde(with = "ImVec4Def")] pub text: ImVec4,
    #[serde(with = "ImVec4Def")] pub text_disabled: ImVec4,
    #[serde(with = "ImVec4Def")] pub window_bg: ImVec4,
    #[serde(with = "ImVec4Def")] pub child_window_bg: ImVec4,
    #[serde(with = "ImVec4Def")] pub popup_bg: ImVec4,
    #[serde(with = "ImVec4Def")] pub border: ImVec4,
    #[serde(with = "ImVec4Def")] pub border_shadow: ImVec4,
    #[serde(with = "ImVec4Def")] pub frame_bg: ImVec4,
    #[serde(with = "ImVec4Def")] pub frame_bg_hovered: ImVec4,
    #[serde(with = "ImVec4Def")] pub frame_bg_active: ImVec4,
    #[serde(with = "ImVec4Def")] pub title_bg: ImVec4,
    #[serde(with = "ImVec4Def")] pub title_bg_collapsed: ImVec4,
    #[serde(with = "ImVec4Def")] pub title_bg_active: ImVec4,
    #[serde(with = "ImVec4Def")] pub menu_bar_bg: ImVec4,
    #[serde(with = "ImVec4Def")] pub scrollbar_bg: ImVec4,
    #[serde(with = "ImVec4Def")] pub scrollbar_grab: ImVec4,
    #[serde(with = "ImVec4Def")] pub scrollbar_grab_hovered: ImVec4,
    #[serde(with = "ImVec4Def")] pub scrollbar_grab_active: ImVec4,
    #[serde(with = "ImVec4Def")] pub combo_bg: ImVec4,
    #[serde(with = "ImVec4Def")] pub check_mark: ImVec4,
    #[serde(with = "ImVec4Def")] pub slider_grab: ImVec4,
    #[serde(with = "ImVec4Def")] pub slider_grab_active: ImVec4,
    #[serde(with = "ImVec4Def")] pub button: ImVec4,
    #[serde(with = "ImVec4Def")] pub button_hovered: ImVec4,
    #[serde(with = "ImVec4Def")] pub button_active: ImVec4,
    #[serde(with = "ImVec4Def")] pub header: ImVec4,
    #[serde(with = "ImVec4Def")] pub header_hovered: ImVec4,
    #[serde(with = "ImVec4Def")] pub header_active: ImVec4,
    #[serde(with = "ImVec4Def")] pub separator: ImVec4,
    #[serde(with = "ImVec4Def")] pub separator_hovered: ImVec4,
    #[serde(with = "ImVec4Def")] pub separator_active: ImVec4,
    #[serde(with = "ImVec4Def")] pub resize_grip: ImVec4,
    #[serde(with = "ImVec4Def")] pub resize_grip_hovered: ImVec4,
    #[serde(with = "ImVec4Def")] pub resize_grip_active: ImVec4,
    #[serde(with = "ImVec4Def")] pub close_button: ImVec4,
    #[serde(with = "ImVec4Def")] pub close_button_hovered: ImVec4,
    #[serde(with = "ImVec4Def")] pub close_button_active: ImVec4,
    #[serde(with = "ImVec4Def")] pub plot_lines: ImVec4,
    #[serde(with = "ImVec4Def")] pub plot_lines_hovered: ImVec4,
    #[serde(with = "ImVec4Def")] pub plot_histogram: ImVec4,
    #[serde(with = "ImVec4Def")] pub plot_histogram_hovered: ImVec4,
    #[serde(with = "ImVec4Def")] pub text_selected_bg: ImVec4,
    #[serde(with = "ImVec4Def")] pub modal_window_darkening: ImVec4,
}

impl From<ImColorsSave> for [ImVec4; 43] {
    fn from(def: ImColorsSave) -> Self {
        [
            def.text,
            def.text_disabled,
            def.window_bg,
            def.child_window_bg,
            def.popup_bg,
            def.border,
            def.border_shadow,
            def.frame_bg,
            def.frame_bg_hovered,
            def.frame_bg_active,
            def.title_bg,
            def.title_bg_collapsed,
            def.title_bg_active,
            def.menu_bar_bg,
            def.scrollbar_bg,
            def.scrollbar_grab,
            def.scrollbar_grab_hovered,
            def.scrollbar_grab_active,
            def.combo_bg,
            def.check_mark,
            def.slider_grab,
            def.slider_grab_active,
            def.button,
            def.button_hovered,
            def.button_active,
            def.header,
            def.header_hovered,
            def.header_active,
            def.separator,
            def.separator_hovered,
            def.separator_active,
            def.resize_grip,
            def.resize_grip_hovered,
            def.resize_grip_active,
            def.close_button,
            def.close_button_hovered,
            def.close_button_active,
            def.plot_lines,
            def.plot_lines_hovered,
            def.plot_histogram,
            def.plot_histogram_hovered,
            def.text_selected_bg,
            def.modal_window_darkening,
        ]
    }
}

impl From<[ImVec4; 43]> for ImColorsSave {
    fn from(def: [ImVec4; 43]) -> Self {
        ImColorsSave {
            text: def[0],
            text_disabled: def[1],
            window_bg: def[2],
            child_window_bg: def[3],
            popup_bg: def[4],
            border: def[5],
            border_shadow: def[6],
            frame_bg: def[7],
            frame_bg_hovered: def[8],
            frame_bg_active: def[9],
            title_bg: def[10],
            title_bg_collapsed: def[11],
            title_bg_active: def[12],
            menu_bar_bg: def[13],
            scrollbar_bg: def[14],
            scrollbar_grab: def[15],
            scrollbar_grab_hovered: def[16],
            scrollbar_grab_active: def[17],
            combo_bg: def[18],
            check_mark: def[19],
            slider_grab: def[20],
            slider_grab_active: def[21],
            button: def[22],
            button_hovered: def[23],
            button_active: def[24],
            header: def[25],
            header_hovered: def[26],
            header_active: def[27],
            separator: def[28],
            separator_hovered: def[29],
            separator_active: def[30],
            resize_grip: def[31],
            resize_grip_hovered: def[32],
            resize_grip_active: def[33],
            close_button: def[34],
            close_button_hovered: def[35],
            close_button_active: def[36],
            plot_lines: def[37],
            plot_lines_hovered: def[38],
            plot_histogram: def[39],
            plot_histogram_hovered: def[40],
            text_selected_bg: def[41],
            modal_window_darkening: def[42],
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImGuiStyleSave {
    pub alpha: f32,
    #[serde(with = "ImVec2Def")] pub window_padding: ImVec2,
    #[serde(with = "ImVec2Def")] pub window_min_size: ImVec2,
    pub window_rounding: f32,
    #[serde(with = "ImVec2Def")] pub window_title_align: ImVec2,
    pub child_window_rounding: f32,
    #[serde(with = "ImVec2Def")] pub frame_padding: ImVec2,
    pub frame_rounding: f32,
    #[serde(with = "ImVec2Def")] pub item_spacing: ImVec2,
    #[serde(with = "ImVec2Def")] pub item_inner_spacing: ImVec2,
    #[serde(with = "ImVec2Def")] pub touch_extra_padding: ImVec2,
    pub indent_spacing: f32,
    pub columns_min_spacing: f32,
    pub scrollbar_size: f32,
    pub scrollbar_rounding: f32,
    pub grab_min_size: f32,
    pub grab_rounding: f32,
    #[serde(with = "ImVec2Def")] pub button_text_align: ImVec2,
    #[serde(with = "ImVec2Def")] pub display_window_padding: ImVec2,
    #[serde(with = "ImVec2Def")] pub display_safe_area_padding: ImVec2,
    pub anti_aliased_lines: bool,
    pub anti_aliased_shapes: bool,
    pub curve_tessellation_tol: f32,
    pub colors: ImColorsSave,
}

impl ImGuiStyleSave {
    pub fn set_style(&self, style: &mut ImGuiStyle) {
        style.alpha = self.alpha;
        style.window_padding = self.window_padding;
        style.window_min_size = self.window_min_size;
        style.window_rounding = self.window_rounding;
        style.window_title_align = self.window_title_align;
        style.child_window_rounding = self.child_window_rounding;
        style.frame_padding = self.frame_padding;
        style.frame_rounding = self.frame_rounding;
        style.item_spacing = self.item_spacing;
        style.item_inner_spacing = self.item_inner_spacing;
        style.touch_extra_padding = self.touch_extra_padding;
        style.indent_spacing = self.indent_spacing;
        style.columns_min_spacing = self.columns_min_spacing;
        style.scrollbar_size = self.scrollbar_size;
        style.scrollbar_rounding = self.scrollbar_rounding;
        style.grab_min_size = self.grab_min_size;
        style.grab_rounding = self.grab_rounding;
        style.button_text_align = self.button_text_align;
        style.display_window_padding = self.display_window_padding;
        style.display_safe_area_padding = self.display_safe_area_padding;
        style.anti_aliased_lines = self.anti_aliased_lines;
        style.anti_aliased_shapes = self.anti_aliased_shapes;
        style.curve_tessellation_tol = self.curve_tessellation_tol;
        style.colors = self.colors.clone().into();
    }
}
