use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, detect_collisions);
	}
}

pub fn detect_collisions(mut collision_events: EventReader<CollisionEvent>) {
  for collision_event in collision_events.read() {
    println!("Received collision event: {:?}", collision_event);
  }
}
