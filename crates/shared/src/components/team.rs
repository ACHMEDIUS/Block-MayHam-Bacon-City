use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Team component
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Team {
    Red,
    Blue,
    None,
}

impl Default for Team {
    fn default() -> Self {
        Self::None
    }
}
