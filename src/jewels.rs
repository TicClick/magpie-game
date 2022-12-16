use bevy::prelude::*;
use rand::prelude::*;

use crate::collision::Collider;

pub const JEWEL_HITBOX: Vec2 = Vec2::new(32.0, 32.0);

#[derive(Component)]
pub struct Jewel;

#[derive(Bundle)]
pub struct JewelBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

const JEWEL_SPRITES: [&str; 2] = ["textures/jewel1.png", "textures/jewel2.png"];

pub fn generate_jewels(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rg = thread_rng();
    let mut last_jewel_pos = Vec3::new(0.0, 0.0, 1.0);

    for _ in 0..100 {
        let path = JEWEL_SPRITES.choose(&mut thread_rng()).unwrap();
        let new_jewel_pos = last_jewel_pos
            + Vec3::new(rg.gen_range(-200.0..200.0), rg.gen_range(100.0..500.0), 0.0);

        commands.spawn((
            JewelBundle {
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation: new_jewel_pos,
                        scale: Vec3::new(2.0, 2.0, 1.0),
                        ..default()
                    },
                    texture: asset_server.load(*path),
                    ..default()
                },
                collider: Collider,
            },
            Jewel,
        ));

        last_jewel_pos = new_jewel_pos;
    }
}
