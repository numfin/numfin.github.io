use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::player::{Player, PLAYER_SIZE};

use super::{Star, STAR_SIZE};

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Component)]
pub struct StarCollectSound;

const COIN_SOUND_FILENAMES: [&str; 5] = [
    "audio/coins/coin1.wav",
    "audio/coins/coin2.wav",
    "audio/coins/coin3.ogg",
    "audio/coins/coin4.wav",
    "audio/coins/coin5.ogg",
];

pub fn player_hit_star(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    enemies: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player.get_single() {
        let min_distance = (PLAYER_SIZE + STAR_SIZE) / 2.;

        for (enemy_entity, enemy_transform) in &enemies {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            if distance < min_distance {
                if let Some(sfx_path) = COIN_SOUND_FILENAMES.choose(&mut thread_rng()) {
                    commands.spawn(AudioBundle {
                        source: asset_server.load(*sfx_path),
                        settings: PlaybackSettings::ONCE,
                    });
                    commands.entity(enemy_entity).despawn();
                    score.value += 1;
                }
            }
        }
    }
}
