use bevy::input::gamepad::{
	GamepadConnection,
	GamepadEvent,
};
use bevy::prelude::*;

pub enum PressState {
	Unpressed,
	JustPressed,
	HeldDown,
	Released,
}

#[derive(Eq, PartialEq)]
enum InputMode {
	Unknown,
	Keyboard,
	Gamepad,
}

#[derive(Resource)]
pub struct Input {
	jump: PressState,
	mode: InputMode,
	x: f32,
}

impl Input {
	pub fn x(&self) -> f32 { self.x }
	pub fn is_jumping(&self) -> bool {
		match self.jump {
			PressState::JustPressed | PressState::HeldDown => true,
			PressState::Unpressed   | PressState::Released => false,
		}
	}
}

pub struct InputPlugin;

fn keyboard_input_event(
	keyboard: Res<ButtonInput<KeyCode>>,
	mut input: ResMut<Input>
) {
	if input.mode != InputMode::Gamepad  { input.mode = InputMode::Keyboard }
	if input.mode == InputMode::Keyboard {
		let moving_right = keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD);
		let moving_left  = keyboard.pressed(KeyCode::ArrowLeft)  || keyboard.pressed(KeyCode::KeyA);

		if       moving_right                 { input.x =  1.; }
		else if  moving_left                  { input.x = -1.; }
		else if !moving_right && !moving_left { input.x =  0.; }

		if       keyboard.just_pressed(KeyCode::Space)  { input.jump = PressState::JustPressed; }
		else if  keyboard.pressed(KeyCode::Space)       { input.jump = PressState::HeldDown;    }
		else if  keyboard.just_released(KeyCode::Space) { input.jump = PressState::Released;    }
		else if !keyboard.pressed(KeyCode::Space)       { input.jump = PressState::Unpressed;   }
	}
}

fn gamepad_connections(
	mut input: ResMut<Input>,
	mut evr_gamepad: EventReader<GamepadEvent>,
) {
	for ev in evr_gamepad.read() {
		let GamepadEvent::Connection(e) = ev else { continue; };

		match &e.connection {
			GamepadConnection::Connected(_) => input.mode = InputMode::Gamepad,
			GamepadConnection::Disconnected => input.mode = InputMode::Unknown,
		}
	}
}

fn gamepad_input_events(
	mut input: ResMut<Input>,
	mut evr_gamepad: EventReader<GamepadEvent>,
) {
	if input.mode == InputMode::Gamepad {
		for ev in evr_gamepad.read() {
			match ev {
				GamepadEvent::Axis(axis) => {
					if axis.axis_type == GamepadAxisType::LeftStickX { input.x = axis.value; }
				}
				GamepadEvent::Button(btn) => {
					// The "value" of a button is typically `0.0` or `1.0`, but it
					// is a `f32` because some gamepads may have buttons that are
					// pressure-sensitive or otherwise analog somehow.
					if btn.button_type == GamepadButtonType::South && btn.value > 0. {
						if !input.is_jumping() { input.jump = PressState::JustPressed; }
						else { input.jump = PressState::HeldDown; }
					} else if btn.button_type == GamepadButtonType::South && btn.value == 0. {
						if input.is_jumping() { input.jump = PressState::Released; }
						else { input.jump = PressState::Unpressed; }
					}
				}
				_ => ()
			}
		}
	}
}

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(Input {
				jump: PressState::Unpressed,
				mode: InputMode::Unknown,
				x: 0.,
			})
			.add_systems(Update, gamepad_connections)
			.add_systems(Update, gamepad_input_events)
			.add_systems(Update, keyboard_input_event);
	}
}
