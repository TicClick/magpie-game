use bevy::prelude::*;

use crate::player;

const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 200.0, 0.0);

#[allow(clippy::type_complexity)]
pub fn focus_camera(
    mut transforms: ParamSet<(
        Query<&Transform, With<player::Player>>,
        Query<&mut Transform, With<Camera2d>>,
    )>,
) {
    let mut player_tr = *transforms.p0().single();
    player_tr.translation += CAMERA_OFFSET;
    for mut transform in transforms.p1().iter_mut() {
        *transform = player_tr;
    }
}
