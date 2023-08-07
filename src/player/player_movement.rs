use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::controls::Controls;

use super::{Player, PLAYER_SPEED};

pub fn player_movement(
    controls: Query<&ActionState<Controls>>,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player.get_single_mut() {
        let controls = controls.single();

        let mut direction = Vec3::ZERO;

        if controls.pressed(Controls::Up) {
            direction += Vec3::new(0., 1., 0.);
        }
        if controls.pressed(Controls::Down) {
            direction += Vec3::new(0., -1., 0.);
        }
        if controls.pressed(Controls::Left) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if controls.pressed(Controls::Right) {
            direction += Vec3::new(1., 0., 0.);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}
