use mlua::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Weapon configuration loaded from Lua script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponScript {
    pub name: String,
    pub damage: f32,
    pub fire_rate: f32,
    pub ammo: u32,
    pub max_ammo: u32,
    pub reload_time: f32,
    pub accuracy: f32,
    pub recoil: f32,
    pub range: f32,
}

impl WeaponScript {
    /// Load weapon configuration from a Lua script
    pub fn from_lua_file(path: &Path) -> LuaResult<Self> {
        let lua = Lua::new();
        let script_content = std::fs::read_to_string(path)
            .map_err(|e| LuaError::RuntimeError(format!("Failed to read weapon script: {e}")))?;

        let table: LuaTable = lua.load(&script_content).eval()?;

        Ok(Self {
            name: table.get("name")?,
            damage: table.get("damage")?,
            fire_rate: table.get("fire_rate")?,
            ammo: table.get("ammo")?,
            max_ammo: table.get("max_ammo")?,
            reload_time: table.get("reload_time")?,
            accuracy: table.get("accuracy")?,
            recoil: table.get("recoil")?,
            range: table.get("range")?,
        })
    }

    /// Load all weapon scripts from a directory
    pub fn load_all(dir: &Path) -> Vec<Self> {
        let mut weapons = Vec::new();

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("lua")
                    && let Ok(weapon) = Self::from_lua_file(&path)
                {
                    weapons.push(weapon);
                }
            }
        }

        weapons
    }
}
