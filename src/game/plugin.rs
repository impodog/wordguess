use super::*;
use crate::prelude::*;
use crate::state::*;

#[derive(Default)]
pub struct GamePlugin;

fn condition_playing(res: Option<Res<GameHandler>>) -> bool {
    if let Some(_) = res {
        return true;
    } else {
        return false;
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                system_spawn_game,
                system_despawn_game,
                system_game_input.run_if(condition_playing),
            ),
        );
    }
}
