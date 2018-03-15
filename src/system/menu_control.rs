use winit::{ElementState, Event, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode,
            WindowEvent, KeyboardInput};
use resource::Input;
use util::Direction;

pub struct MenuGameControlSystem;

impl<'a> ::specs::System<'a> for MenuGameControlSystem {
    type SystemData = (
        ::specs::Fetch<'a, ::resource::Events>,
        ::specs::FetchMut<'a, ::resource::ImGuiOption>,
        ::specs::FetchMut<'a, ::resource::MenuState>,
    );

    fn run(&mut self, (events, mut imgui, mut menu_state): Self::SystemData) {
        let imgui = imgui.as_mut().unwrap();
        imgui.set_mouse_draw_cursor(false);
        for ev in events.0.iter() {
            match *ev {
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input: KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    }, .. },
                    ..
                } => {
                    menu_state.state = ::resource::MenuStateState::Pause;
                },
                Event::WindowEvent {
                    event:
                        WindowEvent::CursorMoved {
                            position: (x, y), ..
                        },
                    ..
                } => imgui.set_mouse_pos(x as f32, y as f32),
                _ => (),
            }
        }
    }
}

pub struct MenuPauseControlSystem {
    mouse_down: [bool; 5],
}

impl MenuPauseControlSystem {
    pub fn new() -> Self {
        MenuPauseControlSystem {
            mouse_down: [false; 5],
        }
    }
}

impl<'a> ::specs::System<'a> for MenuPauseControlSystem {
    type SystemData = (
        ::specs::Fetch<'a, ::resource::Events>,
        ::specs::FetchMut<'a, ::resource::ImGuiOption>,
        ::specs::FetchMut<'a, ::resource::MenuState>,
        ::specs::FetchMut<'a, ::resource::Save>,
        ::specs::FetchMut<'a, ::resource::LevelActions>,
    );

    fn run(&mut self, (events, mut imgui, mut menu_state, mut save, mut level_actions): Self::SystemData) {
        let mut imgui = imgui.as_mut().unwrap();
        imgui.set_mouse_draw_cursor(true);
        send_events_to_imgui(&events, &mut imgui, &mut self.mouse_down);

        match menu_state.state {
            ::resource::MenuStateState::Game => unreachable!(),
            ::resource::MenuStateState::Restart => {
                if menu_state.restart_later_button {
                    menu_state.state = ::resource::MenuStateState::Pause;
                }
            }
            ::resource::MenuStateState::Input(input) => {
                for ev in events.0.iter() {
                    let received_input = match *ev {
                        Event::WindowEvent {
                            event: WindowEvent::KeyboardInput { input: KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            }, .. },
                            ..
                        } => {
                            menu_state.state = ::resource::MenuStateState::Pause;
                            break;
                        },
                        Event::WindowEvent {
                            event: WindowEvent::KeyboardInput { input: KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state: ElementState::Pressed,
                                ..
                            }, .. },
                            ..
                        } => {
                            ::resource::PossibleInput::VirtualKeyCode(keycode)
                        },
                        Event::WindowEvent {
                            event: WindowEvent::MouseInput {
                                button,
                                state: ElementState::Pressed,
                                ..
                            },
                            ..
                        } => {
                            ::resource::PossibleInput::MouseButton(button)
                        },
                        _ => {
                            continue;
                        },
                    };

                    save.set_input(input, received_input);
                    menu_state.state = ::resource::MenuStateState::Pause;
                }
            },
            ::resource::MenuStateState::Pause => {
                for ev in events.0.iter() {
                    match *ev {
                        Event::WindowEvent {
                            event: WindowEvent::KeyboardInput { input: KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            }, .. },
                            ..
                        } => {
                            menu_state.state = ::resource::MenuStateState::Game;
                        },
                        _ => (),
                    }
                }

                if menu_state.continue_button {
                    menu_state.state = ::resource::MenuStateState::Game;
                }

                if menu_state.return_hall_button {
                    level_actions.0.push(::resource::LevelAction::ReturnHall);
                    menu_state.state = ::resource::MenuStateState::Game;
                }

                if menu_state.set_shoot_button {
                    menu_state.state = ::resource::MenuStateState::Input(Input::Shoot);
                }

                if menu_state.set_forward_button {
                    menu_state.state = ::resource::MenuStateState::Input(Input::Direction(Direction::Forward));
                }

                if menu_state.set_backward_button {
                    menu_state.state = ::resource::MenuStateState::Input(Input::Direction(Direction::Backward));
                }

                if menu_state.set_left_button {
                    menu_state.state = ::resource::MenuStateState::Input(Input::Direction(Direction::Left));
                }

                if menu_state.set_right_button {
                    menu_state.state = ::resource::MenuStateState::Input(Input::Direction(Direction::Right));
                }

                if menu_state.reset_button {
                    save.reset_controls();
                    menu_state.mouse_sensibility_input = save.mouse_sensibility();
                    menu_state.field_of_view_slider = save.field_of_view();
                }

                if menu_state.fullscreen_checkbox {
                    save.toggle_fullscreen();
                    menu_state.state = ::resource::MenuStateState::Restart;
                }

                save.set_mouse_sensibility_lazy(menu_state.mouse_sensibility_input);

                if save.set_vulkan_device_uuid_lazy(&menu_state.vulkan_device) {
                    menu_state.state = ::resource::MenuStateState::Restart;
                }

                save.set_effect_volume_lazy(menu_state.music_volume_slider);
                save.set_music_volume_lazy(menu_state.effect_volume_slider);
                save.set_field_of_view_lazy(menu_state.field_of_view_slider);
            },
        }
    }
}

fn send_events_to_imgui(events: &::resource::Events, imgui: &mut ::imgui::ImGui, mouse_down: &mut [bool; 5]) {
    for ev in events.0.iter() {
        match *ev {
            Event::WindowEvent {
                event: WindowEvent::MouseInput { button, state, .. },
                ..
            } => {
                match button {
                    MouseButton::Left => mouse_down[0] = state == ElementState::Pressed,
                    MouseButton::Right => mouse_down[1] = state == ElementState::Pressed,
                    MouseButton::Middle => mouse_down[2] = state == ElementState::Pressed,
                    MouseButton::Other(0) => {
                        mouse_down[3] = state == ElementState::Pressed
                    }
                    MouseButton::Other(1) => {
                        mouse_down[4] = state == ElementState::Pressed
                    }
                    MouseButton::Other(_) => (),
                }
                imgui.set_mouse_down(&mouse_down);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::CursorMoved {
                        position: (x, y), ..
                    },
                ..
            } => imgui.set_mouse_pos(x as f32, y as f32),
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
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
                    Some(VirtualKeyCode::LControl) | Some(VirtualKeyCode::RControl) => {
                        imgui.set_key_ctrl(pressed)
                    }
                    Some(VirtualKeyCode::LShift) | Some(VirtualKeyCode::RShift) => {
                        imgui.set_key_shift(pressed)
                    }
                    Some(VirtualKeyCode::LAlt) | Some(VirtualKeyCode::RAlt) => {
                        imgui.set_key_alt(pressed)
                    }
                    Some(VirtualKeyCode::LWin) | Some(VirtualKeyCode::RWin) => {
                        imgui.set_key_super(pressed)
                    }
                    _ => (),
                }
            }
            Event::WindowEvent {
                event:
                    WindowEvent::MouseWheel {
                        delta,
                        phase: TouchPhase::Moved,
                        ..
                    },
                ..
            } => {
                // TODO: does both are send ? does it depend on computer
                match delta {
                    MouseScrollDelta::LineDelta(_, y) => imgui.set_mouse_wheel(y),
                    MouseScrollDelta::PixelDelta(_, y) => imgui.set_mouse_wheel(y),
                }
            }
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter(c),
                ..
            } => imgui.add_input_character(c),
            _ => (),
        }
    }
}
