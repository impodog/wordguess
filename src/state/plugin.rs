use super::{EndGameEvent, GameState, StartGameEvent};
use crate::prelude::*;

pub struct StatePlugin;

fn system_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system_camera)
            .add_event::<StartGameEvent>()
            .add_event::<EndGameEvent>()
            .add_state::<GameState>();
    }
}
