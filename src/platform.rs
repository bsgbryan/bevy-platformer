use bevy::prelude::*;

use crate::surfaces::ground::GroundBundle;

pub struct PlatformPlugin;

fn init(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
	GroundBundle::new(
		&mut commands,
		Vec2::new(0., -200.),
		IVec2::new(7, 5),
		&asset_server,
		&mut texture_atlas_layouts,
	);
}

impl Plugin for PlatformPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init);
	}
}
