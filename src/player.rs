use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::animation::AnimationTimer;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

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
	if query.is_empty() {
		return;
	}

	let (player, output) = query.single();

	if output.desired_translation.x > 0.0 {
		commands.entity(player).insert(Direction::Right);
	} else if output.desired_translation.x < 0.0 {
		commands.entity(player).insert(Direction::Left);
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
					translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 80.0, 0.0),
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
		))
		.insert(RigidBody::KinematicPositionBased)
		.insert(Collider::cuboid(6.0, 5.0))
		.insert(KinematicCharacterController::default())
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
