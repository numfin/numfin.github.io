use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
            .add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        // transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 1.),
        ..Default::default()
    });
}
