use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::input::Input;

const PLAYER_VELOCITY_X: f32 = 250.;

const MAX_JUMP_HEIGHT: f32 = 128.;
const JUMP_LERP_FACTOR: f32 = 8.;
const MIN_JUMP_MOVE: f32 = 0.33;

#[derive(Component)]
struct Jump(f32, f32);

#[derive(Component)]
pub struct DownForce(pub f32, pub f32, pub u8);

fn jump(
	mut input: ResMut<Input>,
	mut commands: Commands,
	mut query: Query<(
		Entity,
		&KinematicCharacterControllerOutput,
		&mut DownForce,
	), (
		With<KinematicCharacterController>,
		Without<Jump>,
	)>,
) {
	let Ok((player, output, mut df)) = query.get_single_mut() else { return };

	if input.is_jumping() && (output.grounded || df.2 < 12) {
		commands
			.entity(player)
			.insert(Jump(0.0, output.effective_translation.y));

		df.1 = 0.;
	}
}

fn rise(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(
		Entity,
		&mut KinematicCharacterController,
		&mut Jump,
		&mut DownForce,
	)>,
) {
	let Ok((entity, mut player, mut jump, mut df)) = query.
		get_single_mut() else { return };

	let movement = {
		if jump.0 >= MAX_JUMP_HEIGHT {
			commands.entity(entity).remove::<Jump>();

			MAX_JUMP_HEIGHT - jump.0
		}
		else {
			let foo = {
				if jump.0 < MAX_JUMP_HEIGHT * 0.75 { 1. }
				else { 1. + (2.000001 - (MAX_JUMP_HEIGHT / jump.0)) }
			};

			let m = jump.0.lerp(
				jump.1 + MAX_JUMP_HEIGHT,
				time.delta_seconds() * JUMP_LERP_FACTOR * foo,
			) - jump.0;

			if m < MIN_JUMP_MOVE { MIN_JUMP_MOVE }
			else                 { m }
		}
	};

	if movement == 0. { commands.entity(entity).remove::<Jump>(); }
	else              { jump.0 += movement; df.1 = 0.; }

	match player.translation {
		Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
		None            => player.translation = Some(Vec2::new(0., movement)),
	}
}

fn gravity(
	time: Res<Time>,
	mut query: Query<(
		&mut KinematicCharacterController,
		&KinematicCharacterControllerOutput,
		&mut DownForce,
	)>,
) {
	let Ok((mut player, output, mut df)) = query.get_single_mut() else { return };

	let factor = {
		let suspend_gravity = df.2 < 2 || output.desired_translation.y > 0.;
		if suspend_gravity { 0.00001 }
		else         			 { df.0    }
	};

	let movement = df.1.lerp(factor, time.delta_seconds() * JUMP_LERP_FACTOR);

	df.1 = movement;

	match player.translation {
		Some(vec) => player.translation = Some(Vec2::new(vec.x, -movement)),
		None            => player.translation = Some(Vec2::new(0., -movement)),
	}

	if !output.grounded {
		if let Some(v) = df.2.checked_add(1) { df.2 = v; }
	}
	else { df.2 = 0; }
}

fn movement(
	input: Res<Input>,
	time: Res<Time>,
	mut query: Query<&mut KinematicCharacterController>,
) {
	let Ok(mut player) = query.get_single_mut() else { return };
	let movement = time.delta_seconds() * PLAYER_VELOCITY_X * input.x();

	match player.translation {
		Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)),
		None => player.translation = Some(Vec2::new(movement, 0.0)),
	}
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, movement)
			.add_systems(Update, jump)
			.add_systems(Update, rise)
			.add_systems(Update, gravity);
	}
}
