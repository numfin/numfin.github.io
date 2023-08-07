use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{Enemy, ENEMY_SIZE};

#[derive(Component)]
pub struct WallHitSound;

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

        if translation.x >= window.width() - enemy_size_half {
            enemy.direction.x = -1.;
            direction_changed = true;
        }
        if translation.x <= enemy_size_half {
            enemy.direction.x = 1.;
            direction_changed = true;
        }
        if translation.y >= window.height() - enemy_size_half {
            enemy.direction.y = -1.;
            direction_changed = true;
        }
        if translation.y <= enemy_size_half {
            enemy.direction.y = 1.;
            direction_changed = true;
        }

        if direction_changed {
            commands.spawn((
                AudioBundle {
                    source: asset_server.load("audio/hitwall.wav"),
                    settings: PlaybackSettings::ONCE,
                },
                WallHitSound,
            ));
        }
    }
}
pub fn play_sound(query_music: Query<&AudioSink, With<WallHitSound>>) {
    if let Ok(sink) = query_music.get_single() {
        sink.play()
    }
}
