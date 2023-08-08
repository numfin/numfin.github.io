use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{Enemy, ENEMY_SIZE};

pub fn limit_enemy_movement(
    mut enemies: Query<&mut Transform, With<Enemy>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();

    for mut transform in &mut enemies {
        let mut translation = transform.translation;
        let enemy_size_half = ENEMY_SIZE / 2.;
        let half_w = (window.width() / 2.) - enemy_size_half;
        let half_h = (window.height() / 2.) - enemy_size_half;

        translation.x = translation.x.max(-half_w).min(half_w);
        translation.y = translation.y.max(-half_h).min(half_h);

        transform.translation = translation;
    }
}
