use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;

const COLOR_FLOOR: Color = Color::srgb(0.45, 0.55, 0.66);
const COLOR_PLATFORM: Color = Color::srgb(0.29, 0.31, 0.41);

const FLOOR_THICKNESS: f32 = 10.0;

#[derive(Bundle)]
struct PlatformBundle {
	sprite_bundle: SpriteBundle,
	body: RigidBody,
	collider: Collider,
}

impl PlatformBundle {
	fn new(x: f32, scale: Vec3) -> Self {
		Self {
			sprite_bundle: SpriteBundle {
				sprite: Sprite {
					color: COLOR_PLATFORM,
					..Default::default()
				},
				transform: Transform {
					translation: Vec3::new(x, WINDOW_BOTTOM_Y + (scale.y / 2.0) + FLOOR_THICKNESS, 0.0),
					scale,
					..Default::default()
				},
				..Default::default()
			},
			body: RigidBody::Fixed,
			collider: Collider::cuboid(0.5, 0.5),
		}
	}
}

pub struct PlatformPlugin;

fn foo(mut commands: Commands) {
	commands.spawn(PlatformBundle::new(-100.0, Vec3::new(75.0, 200.0, 1.0)));
	commands.spawn(PlatformBundle::new(100.0, Vec3::new(50.0, 350.0, 1.0)));
	commands.spawn(PlatformBundle::new(350.0, Vec3::new(150.0, 250.0, 1.0)));

	commands
		.spawn(SpriteBundle {
			sprite: Sprite {
				color: COLOR_FLOOR,
				..Default::default()
			},
			transform: Transform {
				translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (FLOOR_THICKNESS / 2.0), 0.0),
				scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
				..Default::default()
			},
			..Default::default()
		})
		.insert(RigidBody::Fixed)
		.insert(Collider::cuboid(0.5, 0.5));
}

impl Plugin for PlatformPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, foo);
	}
}
