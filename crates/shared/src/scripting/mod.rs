//! Lua scripting integration

mod weapon_script;

pub use weapon_script::*;

use mlua::prelude::*;
use std::path::Path;

/// Load and execute a Lua script file
pub fn load_script(path: &Path) -> LuaResult<Lua> {
    let lua = Lua::new();
    let script_content = std::fs::read_to_string(path)
        .map_err(|e| LuaError::RuntimeError(format!("Failed to read script: {e}")))?;
    lua.load(&script_content).exec()?;
    Ok(lua)
}

/// Load a Lua table from a script file
pub fn load_table(path: &Path) -> LuaResult<LuaTable> {
    let lua = Lua::new();
    let script_content = std::fs::read_to_string(path)
        .map_err(|e| LuaError::RuntimeError(format!("Failed to read script: {e}")))?;
    lua.load(&script_content).eval()
}
