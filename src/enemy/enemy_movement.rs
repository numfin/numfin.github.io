use bevy::prelude::*;

use super::{Enemy, ENEMY_SPEED};

pub fn enemy_movement(mut enemies: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in &mut enemies {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}
