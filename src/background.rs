use bevy::prelude::*;
use bevy_parallax::{
    LayerData, LayerSpeed, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxResource,
};

pub struct BackgroundPlugin;

pub fn make_background_layers() -> ParallaxResource {
    ParallaxResource {
        layer_data: vec![
            LayerData {
                speed: LayerSpeed::Bidirectional(1.1, 1.1),
                path: "textures/background/forest.png".to_string(),
                tile_size: Vec2::new(256.0, 256.0),
                cols: 1,
                rows: 1,
                scale: 5.0,
                z: -1.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
                path: "textures/background/clouds.png".to_string(),
                tile_size: Vec2::new(200.0, 200.0),
                cols: 1,
                rows: 1,
                scale: 10.0,
                z: -0.5,
                ..default()
            },
        ],
        ..default()
    }
}

pub fn initialize_camera_system(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(ParallaxCameraComponent);
}

pub fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(1.0, 0.0),
        });
    } else if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(-1.0, 0.0),
        });
    }
    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(0.0, 1.0),
        });
    } else if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(0.0, -1.0),
        });
    }
}
