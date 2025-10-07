use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Weapon component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub damage: f32,
    pub fire_rate: f32,
    pub ammo: u32,
    pub max_ammo: u32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self { damage: 25.0, fire_rate: 0.5, ammo: 30, max_ammo: 30 }
    }
}
