use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{Enemy, ENEMY_SIZE};

pub fn update_enemy_rotation(
    mut commands: Commands,
    mut enemies: Query<(&Transform, &mut Enemy)>,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window.single();

    for (transform, mut enemy) in &mut enemies {
        let translation = transform.translation;
        let enemy_size_half = ENEMY_SIZE / 2.;
        let mut direction_changed = false;
        let half_w = (window.width() / 2.) - enemy_size_half;
        let half_h = (window.height() / 2.) - enemy_size_half;

        if translation.x >= half_w {
            enemy.direction.x = -1.;
            direction_changed = true;
        }
        if translation.x <= -half_w {
            enemy.direction.x = 1.;
            direction_changed = true;
        }
        if translation.y >= half_h {
            enemy.direction.y = -1.;
            direction_changed = true;
        }
        if translation.y <= -half_h {
            enemy.direction.y = 1.;
            direction_changed = true;
        }

        if direction_changed {
            commands.spawn((AudioBundle {
                source: asset_server.load("audio/hitwall.wav"),
                settings: PlaybackSettings::ONCE,
            },));
        }
    }
}
