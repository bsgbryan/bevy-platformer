use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const COLOR_PLATFORM: Color = Color::srgb(0.29, 0.31, 0.41);

#[derive(Bundle, Default)]
pub struct GroundBundle {
	transform: Transform,
	collider: Collider,
}

pub const SCALE: f32 = 16. * 4.;

impl GroundBundle {
	pub fn new(
		commands: &mut Commands,
		position: Vec2,
		size: IVec2,
		asset_server: &Res<AssetServer>,
		texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
	) {
		let texture = asset_server.load("spritesheets/surfaces.png");
		let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 30, 3, None, None);
		let texture_atlas_layout = texture_atlas_layouts.add(layout);

		let mut sprites: Vec<(SpriteBundle, TextureAtlas)> = vec![];

		for i in 0..size.x * size.y {
			let x = i % size.x;
			let y = (i / size.x) % size.y;

			let half_x = ((size.x - 1) as f32 * 0.5) * SCALE;
			let iter_x = x as f32 * SCALE;
			let offset_x = iter_x - half_x;
			let pos_x = offset_x;

			let half_y = ((size.y - 1) as f32 * 0.5) * SCALE;
			let iter_y = y as f32 * SCALE;
			let offset_y = iter_y - half_y;
			let pos_y = -offset_y;

			let sprite_x = {
				if x < size.x - 1 && x != 0 { 1 }
				else if x == size.x - 1 { 2 }
				else { 0 }
			};
			let sprite_y =  {
				if y < size.y - 1 && y != 0 { 1 }
				else if y == size.y - 1 { 2 }
				else { 0 }
			};

			let index = sprite_x + (sprite_y * 30);

			sprites.push((
				SpriteBundle {
					sprite: Sprite {
						color: COLOR_PLATFORM,
						..Default::default()
					},
					texture: texture.clone(),
					transform: Transform {
						scale: Vec3::new(4., 4., 1.),
						translation: Vec3::new(pos_x, pos_y, 0.),
						..Default::default()
					},
					..Default::default()
				},
				TextureAtlas {
					index: index as usize,
					layout: texture_atlas_layout.clone(),
				},
			));
		}

		commands.spawn((
			SpatialBundle::from_transform(
				Transform {
					translation: Vec3::new(position.x, position.y, 0.),
					..Default::default()
				}),
				Collider::round_cuboid(
					size.x as f32 * SCALE * 0.5 - 6. - 32.,
					size.y as f32 * SCALE * 0.5 - 6. - 32.,
					32.
				),
			))
			.with_children(|parent| {
				for s in sprites {
					parent.spawn(s);
				}
			});

		// commands.spawn((
		// 	SpatialBundle::from_transform(
		// 		Transform {
		// 			translation: Vec3::new(
		// 				position.x,
		// 				position.y - size.y as f32 * 0.5 * SCALE - 5. * SCALE,
		// 				0.,
		// 			),
		// 			..Default::default()
		// 		},
		// 	),
		// 	Collider::cuboid(
		// 		size.x as f32 * SCALE * 0.5,
		// 		10.						* SCALE * 0.5,
		// 	),
		// ));
	}
}
