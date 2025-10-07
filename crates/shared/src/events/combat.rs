use bevy_ecs::prelude::*;

/// Event fired when damage is dealt
#[derive(Event, Debug, Clone)]
pub struct DamageEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: f32,
}

/// Event fired when a player dies
#[derive(Event, Debug, Clone)]
pub struct DeathEvent {
    pub victim: Entity,
    pub killer: Option<Entity>,
}
