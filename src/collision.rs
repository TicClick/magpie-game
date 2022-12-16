use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::prelude::*;

use crate::jewels::{Jewel, JEWEL_HITBOX};
use crate::player::{MagpieCollisionSounds, Player, PLAYER_HITBOX};
use crate::scoreboard::Scoreboard;

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

pub fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    magpie_query: Query<&Transform, With<Player>>,
    collider_query: Query<(Entity, &Transform, Option<&Jewel>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let magpie_transform = magpie_query.single();

    for (collider_entity, transform, maybe_jewel) in &collider_query {
        let collision = collide(
            magpie_transform.translation,
            PLAYER_HITBOX,
            transform.translation,
            JEWEL_HITBOX,
        );
        if collision.is_some() {
            collision_events.send_default();
            scoreboard.score += 1;

            if maybe_jewel.is_some() {
                commands.entity(collider_entity).despawn();
            }
        }
    }
}

pub fn play_collision_sound(
    audio: Res<Audio>,
    sounds: Res<MagpieCollisionSounds>,
    collision_events: EventReader<CollisionEvent>,
) {
    if !collision_events.is_empty() {
        collision_events.clear();
        let sound = sounds.0.choose(&mut thread_rng()).unwrap();
        audio.play(sound.clone());
    }
}

pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
