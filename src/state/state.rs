use crate::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, States, Reflect)]
pub enum GameState {
    Settings,
    Playing,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Settings
    }
}
