use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use super::{Enemy, ENEMIES_AMOUNT};

pub fn spawn_enemies(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window.single();
    for _ in 0..ENEMIES_AMOUNT {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            },
            Enemy {
                direction: Vec2::new(random(), random()).normalize(),
            },
        ));
    }
}
#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2., TimerMode::Repeating),
        }
    }
}

pub fn spawn_enemy_over_time(
    mut commands: Commands,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window.single();
    if spawn_timer.timer.tick(time.delta()).finished() {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            },
            Enemy {
                direction: Vec2::new(random(), random()).normalize(),
            },
        ));
    }
}
