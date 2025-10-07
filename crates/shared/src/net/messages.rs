use bevy_math::Vec3;
use serde::{Deserialize, Serialize};

/// Client input message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInput {
    pub movement: Vec3,
    pub look_direction: Vec3,
    pub fire: bool,
    pub jump: bool,
}

/// Server state update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateUpdate {
    pub tick: u64,
    pub positions: Vec<(u64, Vec3)>,
}
