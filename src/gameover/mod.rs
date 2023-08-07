use bevy::prelude::*;

use crate::stars::collect_stars::Score;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .add_systems(Update, on_game_over);
    }
}

#[derive(Event)]
pub struct GameOverEvent;

fn on_game_over(mut game_over_events: EventReader<GameOverEvent>, score: Res<Score>) {
    for _ in game_over_events.iter() {
        println!("Game over! Your score is: {}", score.value)
    }
}
