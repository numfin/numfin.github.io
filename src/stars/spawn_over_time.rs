use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use super::{Star, STAR_SPAWN_SECS};

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}
impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_SECS, TimerMode::Repeating),
        }
    }
}

pub fn spawn_over_time(
    mut commands: Commands,
    mut spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window.single();
    if spawn_timer.timer.tick(time.delta()).finished() {
        let random_x = (random::<f32>() - 0.5) * window.width();
        let random_y = (random::<f32>() - 0.5) * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}
