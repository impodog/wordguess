use bevy::ecs::schedule::ScheduleLabel;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, States, Reflect, ScheduleLabel)]
pub enum GameState {
    Settings,
    Playing,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Settings
    }
}
