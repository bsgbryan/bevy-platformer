use bevy::{
	prelude::*,
	sprite::MaterialMesh2dBundle,
};

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const COLOR_PLATFORM: Color = Color::srgb(0.29, 0.31, 0.41);
const COLOR_PLAYER: Color = Color::srgb(0.60, 0.55, 0.60);

pub struct PlatformPlugin;

fn foo(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands.spawn(SpriteBundle {
		sprite: Sprite {
			color: COLOR_PLATFORM,
			..Default::default()
		},
		transform: Transform {
			translation: Vec3::new(-100.0, WINDOW_BOTTOM_Y + (200.0 / 2.0), 0.0),
			scale: Vec3::new(75.0, 200.0, 1.0),
			..Default::default()
		},
		..Default::default()
	});

	commands.spawn(SpriteBundle {
		sprite: Sprite {
			color: COLOR_PLATFORM,
			..Default::default()
		},
		transform: Transform {
			translation: Vec3::new(100.0, WINDOW_BOTTOM_Y + (350.0 / 2.0), 0.0),
			scale: Vec3::new(50.0, 350.0, 1.0),
			..Default::default()
		},
		..Default::default()
	});

	commands.spawn(SpriteBundle {
		sprite: Sprite {
			color: COLOR_PLATFORM,
			..Default::default()
		},
		transform: Transform {
			translation: Vec3::new(350.0, WINDOW_BOTTOM_Y + (250.0 / 2.0), 0.0),
			scale: Vec3::new(150.0, 250.0, 1.0),
			..Default::default()
		},
		..Default::default()
	});

	commands.spawn(Camera2dBundle::default());

	commands.spawn(MaterialMesh2dBundle {
		mesh: meshes.add(Circle::default()).into(),
		material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
		transform: Transform {
			translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 15.0, 0.0),
			scale: Vec3::new(30.0, 30.0, 1.0),
			..Default::default()
		},
		..default()
	});
}

impl Plugin for PlatformPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, foo);
	}
}
