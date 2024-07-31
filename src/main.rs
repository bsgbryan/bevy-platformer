use bevy::{
	prelude::*,
	window::WindowResolution,
};
use bevy_rapier2d::prelude::*;

use bevy_platformer::{
	// hello::HelloPlugin,
	animation::AnimationPlugin,
	camera::CameraPlugin,
	movement::MovementPlugin,
	platform::PlatformPlugin,
	player::PlayerPlugin,
	player_animation::PlayerAnimationPlugin,
};

const COLOR_BACKGROUND: Color = Color::srgb(0.13, 0.13, 0.23);

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
	App::new()
		.insert_resource(ClearColor(COLOR_BACKGROUND))
		.add_plugins((
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Bevy Platformer".to_string(),
						resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
						resizable: false,
						..Default::default()
					}),
					..Default::default()
				})
				.set(ImagePlugin::default_nearest()),
			// HelloPlugin,
			RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0), // Physics plugin
			// RapierDebugRenderPlugin::default(), // Debug plugin
			CameraPlugin,
			AnimationPlugin,
			PlayerPlugin,
			PlatformPlugin,
			MovementPlugin,
			PlayerAnimationPlugin,
		))
		.run();
}
