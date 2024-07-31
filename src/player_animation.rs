use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
	animation::AnimationIndices,
	player::Direction,
};

pub struct PlayerAnimationPlugin;

fn apply_movement_animation(
	mut commands: Commands,
	query: Query<(
		Entity,
		&KinematicCharacterControllerOutput
	), Without<AnimationIndices>>,
) {
	if query.is_empty() {
		return;
	}

	let (player, output) = query.single();
	if output.desired_translation.x != 0.0 && output.grounded {
		commands
			.entity(player)
			.insert(AnimationIndices { first: 301, last: 304 });
	}
}

fn remove_movement_animation(
	mut commands: Commands,
	mut query: Query<(
		Entity,
		&KinematicCharacterControllerOutput,
		&mut TextureAtlas,
	), With<AnimationIndices>,
	>
) {
	if query.is_empty() { return; }

	let (player, output, mut atlas) = query.single_mut();
	if output.desired_translation.x == 0.0 && output.grounded {
		commands
			.entity(player)
			.remove::<AnimationIndices>();
		atlas.index = 301;
	}
}

fn apply_jump_animation(
	mut commands: Commands,
	mut query: Query<(
		Entity,
		&KinematicCharacterControllerOutput,
		&mut TextureAtlas,
	)>
) {
	if query.is_empty() {
		return;
	}

	let (player, output, mut atlas) = query.single_mut();
	if !output.grounded {
		commands
			.entity(player)
			.remove::<AnimationIndices>();

		atlas.index = 280;
	}
}

fn apply_idle_animation(
	mut commands: Commands,
	mut query: Query<(
		Entity,
		&KinematicCharacterControllerOutput,
		&mut TextureAtlas,
	)>
) {
	if query.is_empty() {
		return;
	}

	let (player, output, mut atlas) = query.single_mut();
	if output.desired_translation.x == 0.0 && output.grounded {
		commands
			.entity(player)
			.remove::<AnimationIndices>();

		atlas.index = 300;
	}
}

fn update_sprite_direction(mut query: Query<(
	&mut Sprite,
	&Direction,
)>) {
	if query.is_empty() { return; }

	let (mut sprite, direction) = query.single_mut();

	match direction {
		Direction::Right => sprite.flip_x = false,
		Direction::Left => sprite.flip_x = true,
	}
}

impl Plugin for PlayerAnimationPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, apply_movement_animation)
			.add_systems(Update, remove_movement_animation)
			.add_systems(Update, apply_jump_animation)
			.add_systems(Update, apply_idle_animation)
			.add_systems(Update, update_sprite_direction);
	}
}
