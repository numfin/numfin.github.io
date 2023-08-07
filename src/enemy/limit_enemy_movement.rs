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

        translation.x = translation.x.max(enemy_size_half);
        translation.x = translation.x.min(window.width() - enemy_size_half);
        translation.y = translation.y.max(enemy_size_half);
        translation.y = translation.y.min(window.height() - enemy_size_half);

        transform.translation = translation;
    }
}
