use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
	animation::AnimationTimer,
	movement::DownForce,
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub enum Direction {
	Right,
	Left,
}

fn update_direction(
	mut commands: Commands,
	query: Query<(
		Entity,
		&KinematicCharacterControllerOutput,
	)>,
) {
	let Ok((player, output)) = query.get_single() else { return ;};

	if output.desired_translation.x > 0.0 {
		commands
			.entity(player)
			.insert(Direction::Right);
	} else if output.desired_translation.x < 0.0 {
		commands
			.entity(player)
			.insert(Direction::Left);
	}
}

fn init(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
	let texture = asset_server.load("spritesheets/tiles.png");
	let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 20, 20, None, Some(UVec2::new(0, 3)));
	let texture_atlas_layout = texture_atlas_layouts.add(layout);

	commands
		.spawn((
			Player,
			SpriteBundle {
				transform: Transform {
					translation: Vec3::new(0.0, 0.0, 0.0),
					scale: Vec3::new(4.0, 4.0, 1.0),
					..Default::default()
				},
				texture,
				sprite: Sprite {
					flip_x: false,
					..default()
				},
				..default()
			},
			TextureAtlas {
				layout: texture_atlas_layout,
				index: 301,
			},
			AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
			DownForce(2.5, 0., 0),
		))
		.insert(RigidBody::KinematicPositionBased)
		.insert(Collider::round_cuboid(2.25, 3.25, 8.))
		.insert(KinematicCharacterController {
			snap_to_ground: Some(CharacterLength::Relative(1.)),
			max_slope_climb_angle: 45f32.to_radians(),
			min_slope_slide_angle: 30f32.to_radians(),
			..default()
		})
		.insert(Direction::Right);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, init)
			.add_systems(Update, update_direction);
	}
}
