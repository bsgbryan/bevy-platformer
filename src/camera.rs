use bevy::prelude::*;
use crate::level_builder::ScreenEdge;
use crate::player::Player;

const CAM_LERP_FACTOR: f32 = 2.;

#[derive(Component)]
pub struct Camera;

pub struct CameraPlugin;

fn init(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

fn follow_player(
	mut screen_edge: ResMut<ScreenEdge>,
	mut camera: Query<
		&mut Transform,
		(With<Camera2d>, Without<Player>),
	>,
	player: Query<
		&Transform,
		(With<Player>, Without<Camera2d>),
	>,
	window: Query<&Window>,
	time: Res<Time>,
) {
	let Ok(mut camera) = camera.get_single_mut() else { return };
	let Ok(player)         = player.get_single()     else { return };
	let Ok(window)           = window.get_single()     else { return };

	let Vec3 { x, y, .. } = player.translation;
	let target = Vec3::new(x, y, camera.translation.z);

	// Applies a smooth effect to camera movement using interpolation between
	// the camera position and the player position on the x and y axes.
	// Here we use the in-game time, to get the elapsed time (in seconds)
	// since the previous update. This avoids jittery movement when tracking
	// the player.
	camera.translation = camera
		.translation
		.lerp(target, time.delta_seconds() * CAM_LERP_FACTOR);

	let half_width  = window.size().x * 0.5;
	let half_height = window.size().y * 0.5;

	screen_edge.east  = camera.translation.x + half_width;
	screen_edge.west  = camera.translation.x - half_width;
	screen_edge.north = camera.translation.y - half_height;
	screen_edge.south = camera.translation.y + half_height;
}

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, init)
			.add_systems(Update, follow_player);
	}
}
