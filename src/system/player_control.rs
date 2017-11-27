use winit::{Event, WindowEvent, ElementState, MouseButton, DeviceEvent};
use util::Direction;
use specs::Join;

pub struct PlayerControlSystem;

impl<'a> ::specs::System<'a> for PlayerControlSystem {
    type SystemData = (::specs::ReadStorage<'a, ::component::Player>,
     ::specs::WriteStorage<'a, ::component::Aim>,
     ::specs::WriteStorage<'a, ::component::Shooter>,
     ::specs::WriteStorage<'a, ::component::Momentum>,
     ::specs::Fetch<'a, ::resource::GameEvents>,
     ::specs::Fetch<'a, ::resource::Config>,
     ::specs::FetchMut<'a, ::resource::PlayerControl>);

    fn run(
        &mut self,
        (players, mut aims, mut shooters, mut momentums, events, config, mut player_control): Self::SystemData,
    ) {
        let (_, player_aim, player_shooter, player_momentum) =
            (&players, &mut aims, &mut shooters, &mut momentums)
                .join()
                .next()
                .unwrap();
        for ev in events.0.iter() {
            match *ev {
                Event::WindowEvent {
                    event: WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state,
                        ..
                    },
                    ..
                } => {
                    match state {
                        ElementState::Pressed => player_shooter.set_shoot(true),
                        ElementState::Released => player_shooter.set_shoot(false),
                    }
                }
                Event::DeviceEvent {
                    event: DeviceEvent::Motion { axis: 0, value: dx }, ..
                } => {
                    player_control.pointer[0] += dx as f32 * config.mouse_sensibility;
                }
                Event::DeviceEvent {
                    event: DeviceEvent::Motion { axis: 1, value: dy }, ..
                } => {
                    player_control.pointer[1] += dy as f32 * config.mouse_sensibility;
                    player_control.pointer[1] = player_control.pointer[1].min(::std::f32::consts::FRAC_PI_2).max(
                        -::std::f32::consts::FRAC_PI_2,
                    );
                }
                Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                    let direction = match input.scancode {
                        25 => Some(Direction::Forward),
                        38 => Some(Direction::Left),
                        39 => Some(Direction::Backward),
                        40 => Some(Direction::Right),
                        _ => None,
                    };
                    if let Some(direction) = direction {
                        player_control.directions.retain(|&elt| elt != direction);
                        if let ElementState::Pressed = input.state {
                            player_control.directions.push(direction);
                        }
                    }
                }
                _ => (),
            }
        }

        // TODO: factorise
        player_aim.rotation = ::na::UnitQuaternion::from_rotation_matrix(&(::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -player_control.pointer[0])) *
            ::na::Rotation3::new(::na::Vector3::new(0.0, player_control.pointer[1], 0.0))));

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
            move_vector = (::na::Rotation3::new(::na::Vector3::new(0.0, 0.0, -player_control.pointer[0])) *
                               move_vector)
                .normalize();
            player_momentum.direction = move_vector;
        }
    }
}
