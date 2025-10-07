use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Game configuration resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub tick_rate: f32,
    pub max_players: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self { tick_rate: 64.0, max_players: 16 }
    }
}
