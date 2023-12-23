pub use super::*;
pub use crate::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnInputBoxEvent>()
            .add_event::<DespawnInputBoxEvent>()
            .add_systems(
                Update,
                (
                    system_input_box,
                    system_start_game,
                    system_spawn_input_box,
                    system_despawn_input_box,
                    system_end_game_event_listener,
                    system_start_game_event_listener,
                ),
            );
    }
}
