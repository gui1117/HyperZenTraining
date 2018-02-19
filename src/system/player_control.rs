use winit::{DeviceEvent, ElementState, Event, WindowEvent};
use util::Direction;
use specs::Join;

pub struct PlayerControlSystem;

impl<'a> ::specs::System<'a> for PlayerControlSystem {
    type SystemData = (
        ::specs::ReadStorage<'a, ::component::Player>,
        ::specs::WriteStorage<'a, ::component::Aim>,
        ::specs::WriteStorage<'a, ::component::Shooter>,
        ::specs::WriteStorage<'a, ::component::Momentum>,
        ::specs::Fetch<'a, ::resource::Events>,
        ::specs::Fetch<'a, ::resource::Save>,
        ::specs::FetchMut<'a, ::resource::PlayerControl>,
    );

    fn run(
        &mut self,
        (
            players,
            mut aims,
            mut shooters,
            mut momentums,
            events,
            save,
            mut player_control,
        ): Self::SystemData,
    ) {
        let (_, player_aim, player_shooter, player_momentum) = (
            &players,
            &mut aims,
            &mut shooters,
            &mut momentums,
        ).join()
            .next()
            .unwrap();

        let mut inputs = vec![];
        for ev in events.0.iter() {
            match *ev {
                Event::WindowEvent {
                    event:
                        WindowEvent::MouseInput {
                            button,
                            state,
                            ..
                        },
                    ..
                } => {
                    inputs.extend(save.convert_mouse_button_input(button).iter().map(|b| (b.clone(), state.clone())));
                }
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput {
                        input: ::winit::KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                        ..
                    },
                    ..
                } => {
                    inputs.extend(save.convert_keycode_input(keycode).iter().map(|c| (c.clone(), state.clone())));
                }
                Event::DeviceEvent {
                    event: DeviceEvent::Motion { axis: 0, value: dx },
                    ..
                } => {
                    player_control.pointer[0] += dx as f32 * save.mouse_sensibility();
                }
                Event::DeviceEvent {
                    event: DeviceEvent::Motion { axis: 1, value: dy },
                    ..
                } => {
                    player_control.pointer[1] += dy as f32 * save.mouse_sensibility();
                    player_control.pointer[1] = player_control.pointer[1]
                        .min(::std::f32::consts::FRAC_PI_2)
                        .max(-::std::f32::consts::FRAC_PI_2);
                }
                _ => (),
            }
        }

        for input in inputs {
            match input {
                (::resource::Input::Shoot, state) => player_shooter.set_shoot(state == ElementState::Released),
                (::resource::Input::Direction(direction), state) => {
                    player_control.directions.retain(|&elt| elt != direction);
                    if let ElementState::Pressed = state {
                        player_control.directions.push(direction);
                    }
                }
            }
        }

        player_aim.rotation = ::na::UnitQuaternion::from_rotation_matrix(
            &(::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -player_control.pointer[0]))
                * ::na::Rotation3::new(::na::Vector3::new(0.0, player_control.pointer[1], 0.0))),
        );

        let mut move_vector: ::na::Vector3<f32> = ::na::zero();
        if player_control.directions.is_empty() {
            player_momentum.direction = ::na::zero();
        } else {
            for &direction in &player_control.directions {
                match direction {
                    Direction::Forward => move_vector[0] = 1.0,
                    Direction::Backward => move_vector[0] = -1.0,
                    Direction::Left => move_vector[1] = 1.0,
                    Direction::Right => move_vector[1] = -1.0,
                }
            }
            move_vector =
                (::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -player_control.pointer[0]))
                    * move_vector)
                    .normalize();
            player_momentum.direction = move_vector;
        }
    }
}
