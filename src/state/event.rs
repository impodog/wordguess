use super::GameState;
use crate::prelude::*;

#[derive(Debug, Clone, Event)]
pub struct StartGameEvent {
    pub min: usize,
    pub max: usize,
}

#[derive(Debug, Clone, Event)]
pub struct EndGameEvent;
