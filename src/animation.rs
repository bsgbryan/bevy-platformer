use bevy::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct AnimationIndices {
	pub first: usize,
	pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, animate);
	}
}

fn animate(
	time: Res<Time>,
	mut query: Query<(
		&AnimationIndices,
		&mut AnimationTimer,
		&mut TextureAtlas,
	), With<Player>>,
) {
	for (indices, mut timer, mut atlas) in &mut query {
		timer.tick(time.delta());
		if timer.just_finished() {
			atlas.index = if atlas.index >= indices.last {
				indices.first
			} else if atlas.index < indices.first {
				indices.first
			} else {
				atlas.index + 1
			};
		}
	}
}
