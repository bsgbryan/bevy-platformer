use bevy::prelude::*;

use crate::surfaces::ground::GroundBundle;

pub const GRID_CELL_SIZE: u8 = 64;

#[derive(Resource)]
struct WorldEdge {
	east: f32,
	north: f32,
	south: f32,
	west: f32,
}

impl Default for WorldEdge {
	fn default() -> Self {
		WorldEdge {
			east:  0.,
			north: 0.,
			south: 0.,
			west:  0.,
		}
	}
}

#[derive(Resource)]
pub struct ScreenEdge {
	pub east: f32,
	pub north: f32,
	pub south: f32,
	pub west: f32,
}

impl Default for ScreenEdge {
	fn default() -> Self {
		ScreenEdge {
			east:   512.,
			north: -384.,
			south:  384.,
			west:  -512.,
		}
	}
}


pub struct LevelBuilderPlugin;

fn init(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
	mut world_edge: ResMut<WorldEdge>,
	window: Query<&Window>,
) {
	let Ok(window) = window.get_single() else { return };
	let width = ((window.size().x / GRID_CELL_SIZE as f32).floor() * 0.5).floor() as i32;
	let half_width = width as f32 * 0.5;

	println!("{}  {}", width, half_width);

	world_edge.east =  half_width * GRID_CELL_SIZE as f32;
	world_edge.west = -half_width * GRID_CELL_SIZE as f32;

	GroundBundle::new(
		&mut commands,
		Vec2::new(0., -(GRID_CELL_SIZE as f32 + (GRID_CELL_SIZE as f32 * 1.5))),
		IVec2::new(width, 3),
		&asset_server,
		&mut texture_atlas_layouts,
	);
}

fn genenerate_ground(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
	mut world_edge: ResMut<WorldEdge>,
	screen_edge: Res<ScreenEdge>,
	window: Query<&Window>,
) {
	let Ok(window) = window.get_single() else { return };

	let height:u8 = 3;
	let width: u16 = 7;
	let size = GRID_CELL_SIZE as f32 * width as f32;

	if world_edge.east - screen_edge.east < window.size().x * 0.5 {
		println!("Rendering surface {}", world_edge.east);
		GroundBundle::new(
			&mut commands,
			Vec2::new(
				world_edge.east + (size * 0.5),
				-(GRID_CELL_SIZE as f32 + (GRID_CELL_SIZE as f32 * (height as f32 * 0.5)))
			),
			IVec2::new(width as i32, height as i32),
			&asset_server,
			&mut texture_atlas_layouts,
		);

		world_edge.east += size;
	}
}

impl Plugin for LevelBuilderPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(ScreenEdge::default())
			.insert_resource(WorldEdge::default())
			.add_systems(Startup, init)
			.add_systems(Update, genenerate_ground);
	}
}
