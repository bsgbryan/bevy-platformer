use bevy::prelude::*;

pub enum PressState {
	Unpressed,
	JustPressed,
	HeldDown,
	Released,
}

#[derive(Resource)]
pub struct Input {
	x: f32,
	jump: PressState,
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

fn process(
	keyboard: Res<ButtonInput<KeyCode>>,
	mut input: ResMut<Input>
) {
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

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(Input {
				x: 0.,
				jump: PressState::Unpressed,
			})
			.add_systems(Update, process);
	}
}
