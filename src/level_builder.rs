use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_rapier2d::prelude::*;

use rand_core::RngCore;

use crate::surfaces::ground::{
	SCALE,
	GroundBundle,
};

pub const GRID_CELL_SIZE: f32 = 64.;

#[derive(Resource)]
struct WorldEdge {
	east: f32,
	// north: f32,
	// south: f32,
	west: f32,
}

impl Default for WorldEdge {
	fn default() -> Self {
		WorldEdge {
			east:  0.,
			// north: 0.,
			// south: 0.,
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

const MIN_SIZE: u8 = 2;

pub struct LevelBuilderPlugin;

fn init(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
	mut world_edge: ResMut<WorldEdge>,
	window: Query<&Window>,
) {
	let Ok(window) = window.get_single() else { return };
	let width = ((window.size().x / GRID_CELL_SIZE).floor() * 0.5).floor() as i32;
	let half_width = width as f32 * 0.5;

	world_edge.east =  half_width * GRID_CELL_SIZE;
	world_edge.west = -half_width * GRID_CELL_SIZE;

	GroundBundle::new(
		&mut commands,
		Vec2::new(0., -(GRID_CELL_SIZE + (GRID_CELL_SIZE * 1.5))),
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
	mut rng: ResMut<GlobalEntropy<WyRand>>,
	screen_edge: Res<ScreenEdge>,
	window: Query<&Window>,
) {
	let Ok(window) = window.get_single() else { return };

	if world_edge.east - screen_edge.east < window.size().x * 0.5 {
		let rand_height   =  rng.next_u32() as f32 / u32::MAX as f32;
		let rand_width    =  rng.next_u32() as f32 / u32::MAX as f32;
		let rand_y_offset = (rng.next_u32() as f32 / u32::MAX as f32) - 0.5;
		let rand_gap      =  rng.next_u32() as f32 / u32::MAX as f32;

		let height: u8 = {
			let w = (5. * rand_height).floor() as u8;

			if w < MIN_SIZE { MIN_SIZE }
			else { w }
		};
		let width: u16 = {
			let w = (7. * rand_width).floor() as u16;

			if w < MIN_SIZE as u16 { MIN_SIZE as u16 }
			else { w }
		};

		let size: f32     = GRID_CELL_SIZE * width as f32;
		let y_offset: i16 = ((3. * rand_y_offset).floor() as i16) * GRID_CELL_SIZE as i16;
		let gap: f32       = (1. + (2. * rand_gap).floor()) * GRID_CELL_SIZE;

		let x = world_edge.east + (size * 0.5) + gap as f32;
		let y = -(GRID_CELL_SIZE as f32 + (GRID_CELL_SIZE * (height as f32 * 0.5))) + y_offset as f32;

		if gap > 0. {
			commands.spawn((
				SpatialBundle::from_transform(
					Transform {
						translation: Vec3::new(
							world_edge.east + (gap as f32 * 0.5),
							y - (height as f32 * 0.5 * SCALE) - (2.5 * SCALE),
							0.,
						),
						..Default::default()
					},
				),
				Collider::cuboid(
					gap as f32 * 0.5,
					5. * SCALE * 0.5,
				),
			));
		}

		GroundBundle::new(
			&mut commands,
			Vec2::new(x, y),
			IVec2::new(width as i32, height as i32),
			&asset_server,
			&mut texture_atlas_layouts,
		);

		world_edge.east += size + gap as f32;
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
