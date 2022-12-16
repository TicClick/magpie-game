use bevy::prelude::*;
use rand::prelude::*;

use crate::TIME_STEP;

// Slightly less than the texture because it's not trimmed perfectly
pub const PLAYER_HITBOX: Vec2 = Vec2::new(48.0, 48.0);
const PLAYER_SPEED: Vec2 = Vec2::new(500.0, 500.0);

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct MagpieActiveSounds(pub Vec<Handle<AudioSource>>);

#[derive(Resource)]
pub struct MagpieCollisionSounds(pub Vec<Handle<AudioSource>>);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

const CHIRP_SOUNDS: [&str; 4] = [
    "sounds/chirp1.ogg",
    "sounds/chirp2.ogg",
    "sounds/chirp3.ogg",
    "sounds/chirp4.ogg",
];
const SCREECH_SOUNDS: [&str; 8] = [
    "sounds/screech1.ogg",
    "sounds/screech2.ogg",
    "sounds/screech3.ogg",
    "sounds/screech4.ogg",
    "sounds/screech5.ogg",
    "sounds/screech6.ogg",
    "sounds/screech7.ogg",
    "sounds/screech8.ogg",
];

pub fn add_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_handle = asset_server.load("textures/player-sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(player_handle, Vec2::new(64.0, 64.0), 2, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
    ));
}

pub fn setup_player_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MagpieActiveSounds(
        SCREECH_SOUNDS
            .iter()
            .map(|&p| asset_server.load(p))
            .collect(),
    ));
    commands.insert_resource(MagpieCollisionSounds(
        CHIRP_SOUNDS.iter().map(|&p| asset_server.load(p)).collect(),
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_pos = query.single_mut();

    let mut dx = 0.0;
    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        dx -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        dx += 1.0;
    }

    let mut dy = 0.0;
    if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        dy -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        dy += 1.0;
    }

    let new_x = player_pos.translation.x + dx * PLAYER_SPEED.x * TIME_STEP;
    let new_y = player_pos.translation.y + dy * PLAYER_SPEED.y * TIME_STEP;

    player_pos.translation.x = new_x;
    player_pos.translation.y = new_y;
}

pub fn animate_player_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

pub fn play_active_sound(
    keyboard_input: ResMut<Input<KeyCode>>,
    audio: Res<Audio>,
    sounds: Res<MagpieActiveSounds>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let sound = sounds.0.choose(&mut thread_rng()).unwrap();
        audio.play(sound.clone());
    }
}
