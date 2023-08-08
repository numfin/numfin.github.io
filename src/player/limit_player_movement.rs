use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{Player, PLAYER_SIZE};

pub fn limit_player_movement(
    mut player: Query<&mut Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();
    let half_player_size = PLAYER_SIZE / 2.;

    if let Ok(mut player_transform) = player.get_single_mut() {
        let mut translation = player_transform.translation;
        let half_w = (window.width() / 2.) - half_player_size;
        let half_h = (window.height() / 2.) - half_player_size;

        translation.x = translation.x.max(-half_w).min(half_w);
        translation.y = translation.y.max(-half_h).min(half_h);

        player_transform.translation = translation;
    }
}
