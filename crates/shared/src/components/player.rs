use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Player entity marker
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Player {
    pub id: u64,
}

impl Player {
    #[must_use]
    pub const fn new(id: u64) -> Self {
        Self { id }
    }
}
