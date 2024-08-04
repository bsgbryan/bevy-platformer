use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::input::Input;

const PLAYER_VELOCITY_X: f32 = 250.0;
const PLAYER_VELOCITY_Y: f32 = 600.0;

const MAX_JUMP_HEIGHT: f32 = 230.0;

#[derive(Component)]
struct Jump(f32);

fn jump(
	input: Res<Input>,
	mut commands: Commands,
	query: Query<
		(Entity, &KinematicCharacterControllerOutput),
		(With<KinematicCharacterController>, Without<Jump>),
	>,
) {
	if query.is_empty() {
		return;
	}

	let (player, output) = query.single();

	if input.is_jumping() && output.grounded {
		commands.entity(player).insert(Jump(0.0));
	}
}

fn rise(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
	if query.is_empty() {
		return;
	}

	let (entity, mut player, mut jump) = query.single_mut();

	let mut movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y;

	if movement + jump.0 >= MAX_JUMP_HEIGHT {
		movement = MAX_JUMP_HEIGHT - jump.0;
		commands.entity(entity).remove::<Jump>();
	}

	jump.0 += movement;

	match player.translation {
		Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
		None => player.translation = Some(Vec2::new(0.0, movement)),
	}
}

fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
	if query.is_empty() {
		return;
	}

	let mut player = query.single_mut();

	// I am using two-thirds of the Y-velocity since I want the character to fall slower than it rises
	let movement = time.delta().as_secs_f32() * (PLAYER_VELOCITY_Y / 1.5) * -1.0;

	match player.translation {
		Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
		None => player.translation = Some(Vec2::new(0.0, movement)),
	}
}

fn movement(
	input: Res<Input>,
	time: Res<Time>,
	mut query: Query<&mut KinematicCharacterController>,
) {
	let mut player = query.single_mut();
	let movement = time.delta_seconds() * PLAYER_VELOCITY_X * input.x();

	match player.translation {
		Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)), // update if it already exists
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
			.add_systems(Update, fall);
	}
}
