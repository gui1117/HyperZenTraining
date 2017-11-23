use winit::{Event, WindowEvent, ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode,
            TouchPhase};

pub struct MenuControlSystem {
    mouse_down: [bool; 5],
}

impl MenuControlSystem {
    pub fn new() -> Self {
        MenuControlSystem { mouse_down: [false; 5] }
    }
}

impl<'a> ::specs::System<'a> for MenuControlSystem {
    type SystemData = (::specs::Fetch<'a, ::resource::MenuEvents>,
     ::specs::FetchMut<'a, ::resource::ImGui>);

    fn run(&mut self, (events, mut imgui): Self::SystemData) {
        for ev in events.0.iter() {
            match *ev {
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { button, state, .. }, ..
                } => {
                    match button {
                        MouseButton::Left => self.mouse_down[0] = state == ElementState::Pressed,
                        MouseButton::Right => self.mouse_down[1] = state == ElementState::Pressed,
                        MouseButton::Middle => self.mouse_down[2] = state == ElementState::Pressed,
                        MouseButton::Other(0) => {
                            self.mouse_down[3] = state == ElementState::Pressed
                        }
                        MouseButton::Other(1) => {
                            self.mouse_down[4] = state == ElementState::Pressed
                        }
                        MouseButton::Other(_) => (),
                    }
                    imgui.set_mouse_down(&self.mouse_down);
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseMoved { position: (x, y), .. }, ..
                } => imgui.set_mouse_pos(x as f32, y as f32),
                Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                    let pressed = input.state == ElementState::Pressed;
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::Tab) => imgui.set_key(0, pressed),
                        Some(VirtualKeyCode::Left) => imgui.set_key(1, pressed),
                        Some(VirtualKeyCode::Right) => imgui.set_key(2, pressed),
                        Some(VirtualKeyCode::Up) => imgui.set_key(3, pressed),
                        Some(VirtualKeyCode::Down) => imgui.set_key(4, pressed),
                        Some(VirtualKeyCode::PageUp) => imgui.set_key(5, pressed),
                        Some(VirtualKeyCode::PageDown) => imgui.set_key(6, pressed),
                        Some(VirtualKeyCode::Home) => imgui.set_key(7, pressed),
                        Some(VirtualKeyCode::End) => imgui.set_key(8, pressed),
                        Some(VirtualKeyCode::Delete) => imgui.set_key(9, pressed),
                        Some(VirtualKeyCode::Back) => imgui.set_key(10, pressed),
                        Some(VirtualKeyCode::Return) => imgui.set_key(11, pressed),
                        Some(VirtualKeyCode::Escape) => imgui.set_key(12, pressed),
                        Some(VirtualKeyCode::A) => imgui.set_key(13, pressed),
                        Some(VirtualKeyCode::C) => imgui.set_key(14, pressed),
                        Some(VirtualKeyCode::V) => imgui.set_key(15, pressed),
                        Some(VirtualKeyCode::X) => imgui.set_key(16, pressed),
                        Some(VirtualKeyCode::Y) => imgui.set_key(17, pressed),
                        Some(VirtualKeyCode::Z) => imgui.set_key(18, pressed),
                        Some(VirtualKeyCode::LControl) |
                        Some(VirtualKeyCode::RControl) => imgui.set_key_ctrl(pressed),
                        Some(VirtualKeyCode::LShift) |
                        Some(VirtualKeyCode::RShift) => imgui.set_key_shift(pressed),
                        Some(VirtualKeyCode::LAlt) |
                        Some(VirtualKeyCode::RAlt) => imgui.set_key_alt(pressed),
                        Some(VirtualKeyCode::LWin) |
                        Some(VirtualKeyCode::RWin) => imgui.set_key_super(pressed),
                        _ => (),
                    }

                }
                Event::WindowEvent {
                    event: WindowEvent::MouseWheel {
                        delta,
                        phase: TouchPhase::Moved,
                        ..
                    },
                    ..
                } => {
                    // TODO: does both are send ? does it depend of computer
                    match delta {
                        MouseScrollDelta::LineDelta(_, y) => imgui.set_mouse_wheel(y),
                        MouseScrollDelta::PixelDelta(_, y) => imgui.set_mouse_wheel(y),
                    }
                }
                Event::WindowEvent { event: WindowEvent::ReceivedCharacter(c), .. } => {
                    imgui.add_input_character(c)
                }
                _ => (),
            }
        }
    }
}
