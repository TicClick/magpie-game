use bevy::prelude::*;
use bevy::time::FixedTimestep;

use magpie::background;
use magpie::camera;
use magpie::collision;
use magpie::jewels;
use magpie::player;
use magpie::scoreboard;

use magpie::TIME_STEP;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(bevy_parallax::ParallaxPlugin)
        .insert_resource(background::make_background_layers())
        .insert_resource(scoreboard::Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(background::initialize_camera_system)
        .add_startup_system(player::add_player)
        .add_startup_system(player::setup_player_sounds)
        .add_startup_system(jewels::generate_jewels)
        .add_startup_system(scoreboard::make_scoreboard)
        .add_event::<collision::CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player::move_player)
                .with_system(player::animate_player_sprite)
                .with_system(collision::check_for_collisions.after(player::move_player))
                .with_system(player::play_active_sound)
                .with_system(camera::focus_camera.after(player::move_player))
                .with_system(background::move_camera_system.after(camera::focus_camera))
                .with_system(collision::play_collision_sound.after(collision::check_for_collisions))
                .with_system(collision::update_scoreboard.after(collision::check_for_collisions)),
        )
        .run();
}
