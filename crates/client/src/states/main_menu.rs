use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MainMenuState {
    #[default]
    Main,
    Settings,
}
