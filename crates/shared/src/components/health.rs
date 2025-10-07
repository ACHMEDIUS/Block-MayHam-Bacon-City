use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Health component
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self { current: 100.0, max: 100.0 }
    }
}

impl Health {
    #[must_use]
    pub const fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    #[must_use]
    pub const fn is_alive(&self) -> bool {
        self.current > 0.0
    }
}
