use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const COLOR_PLATFORM: Color = Color::srgb(0.29, 0.31, 0.41);

#[derive(Bundle)]
struct PlatformBundle {
	atlas: TextureAtlas,
	body: RigidBody,
	collider: Collider,
	sprite_bundle: SpriteBundle,
}

impl PlatformBundle {
	fn new(
		position: Vec2,
		index: usize,
		asset_server: &Res<AssetServer>,
		texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
	) -> Self {
		let texture = asset_server.load("spritesheets/tiles.png");
		let layout = TextureAtlasLayout::from_grid(
			UVec2::splat(16),
			20,
			20,
			None,
			None,
		);
		let texture_atlas_layout = texture_atlas_layouts.add(layout);

		Self {
			atlas: TextureAtlas {
				index,
				layout: texture_atlas_layout,
			},
			body: RigidBody::Fixed,
			collider: Collider::cuboid(8.0, 8.0),
			sprite_bundle: SpriteBundle {
				sprite: Sprite {
					color: COLOR_PLATFORM,
					..Default::default()
				},
				texture,
				transform: Transform {
					scale: Vec3::new(4.0, 4.0, 1.0),
					translation: Vec3::new(position.x * 4.0, position.y, 0.0),
					..Default::default()
				},
				..Default::default()
			},
		}
	}
}

pub struct PlatformPlugin;

fn init(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
	commands
		.spawn(PlatformBundle::new(
			Vec2::new(0.0, 0.0),
			103,
			&asset_server,
			&mut texture_atlas_layouts,
		));
	commands
		.spawn(PlatformBundle::new(
			Vec2::new(16.0, 0.0),
			103,
			&asset_server,
			&mut texture_atlas_layouts,
		));
	commands
		.spawn(PlatformBundle::new(
			Vec2::new(32.0, 0.0),
			103,
			&asset_server,
			&mut texture_atlas_layouts,
		));
}

impl Plugin for PlatformPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init);
	}
}
