use bevy::window::WindowMode;
use bevy::{prelude::*, window};

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
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    fit_canvas_to_parent: true,
                    mode: WindowMode::BorderlessFullscreen,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            }),
            CameraPlugin,
            ControlsPlugin,
            PlayerPlugin,
            EnemyPlugin,
            StarsPlugin,
            GameOverPlugin,
        ))
        .run();
}
