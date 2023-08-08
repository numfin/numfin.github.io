use bevy::prelude::*;

use crate::enemy::{Enemy, ENEMY_SIZE};
use crate::gameover::GameOverEvent;

use super::{Player, PLAYER_SIZE};

#[derive(Component)]
pub struct EnemyHitSound;

pub fn enemy_collision(
    mut commands: Commands,
    player: Query<(Entity, &Transform), With<Player>>,
    enemies: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut game_over_events: EventWriter<GameOverEvent>,
) {
    if let Ok((player_entity, player_transform)) = player.get_single() {
        let min_distance = (PLAYER_SIZE + ENEMY_SIZE) / 2.;

        for enemy_transform in &enemies {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            if distance < min_distance {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosion.wav"),
                    settings: PlaybackSettings::ONCE,
                });
                if let Some(mut player_entity) = commands.get_entity(player_entity) {
                    player_entity.despawn();
                    game_over_events.send(GameOverEvent)
                }
            }
        }
    }
}
