use bevy_ecs::prelude::*;
use std::collections::HashMap;

use crate::scripting::WeaponScript;

/// Resource that holds all loaded Lua scripts
#[derive(Resource, Debug, Default)]
pub struct LuaScriptRegistry {
    pub weapons: HashMap<String, WeaponScript>,
}

impl LuaScriptRegistry {
    pub fn new() -> Self {
        Self { weapons: HashMap::new() }
    }

    /// Load all weapon scripts from the scripts/weapons directory
    pub fn load_weapons(&mut self, scripts_dir: &str) {
        let weapons_dir = std::path::Path::new(scripts_dir).join("weapons");
        let weapons = WeaponScript::load_all(&weapons_dir);

        for weapon in weapons {
            self.weapons.insert(weapon.name.clone(), weapon);
        }
    }

    /// Get a weapon script by name
    pub fn get_weapon(&self, name: &str) -> Option<&WeaponScript> {
        self.weapons.get(name)
    }
}
