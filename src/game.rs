use bevy::prelude::*;

use crate::camera::CameraPlugin;
use crate::controls::ControlsPlugin;
use crate::enemy::EnemyPlugin;
use crate::gameover::GameOverPlugin;
use crate::player::PlayerPlugin;
use crate::stars::StarsPlugin;
use crate::system_sets::{LimitMovementSystemSet, MovementSystemSet};

pub fn run() {
    App::new()
        .configure_set(Update, LimitMovementSystemSet.after(MovementSystemSet))
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            ControlsPlugin,
            PlayerPlugin,
            EnemyPlugin,
            StarsPlugin,
            GameOverPlugin,
        ))
        .run();
}
