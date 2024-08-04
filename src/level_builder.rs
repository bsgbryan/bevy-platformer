use bevy::prelude::*;

use crate::{
	player::Player,
	surfaces::ground::GroundBundle,
};

#[derive(Resource)]
struct WorldEdge {
	east: f32,
	north: f32,
	south: f32,
	west: f32,
}

pub struct PlatformPlugin;

fn init(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
	GroundBundle::new(
		&mut commands,
		Vec2::new(0., -200.),
		IVec2::new(7, 3),
		&asset_server,
		&mut texture_atlas_layouts,
	);
}

fn genenerate_ground(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
	mut edge: ResMut<WorldEdge>,
	query: Query<&Transform, With<Player>>
) {
	let Ok(transform) = query.get_single() else { return };
	let t = transform.translation.xy();

	if t.x > edge.east {
		println!("{}, east: {}", t.x, edge.east);
		GroundBundle::new(
			&mut commands,
			Vec2::new(t.x + 300., -200.),
			IVec2::new(7, 3),
			&asset_server,
			&mut texture_atlas_layouts,
		);

		edge.east = t.x + 448.;
	}
}

impl Plugin for PlatformPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(WorldEdge {
				east: 200.,
				north: -200.,
				south: 200.,
				west: -200.,
			})
			.add_systems(Startup, init)
			.add_systems(Update, genenerate_ground);
	}
}
