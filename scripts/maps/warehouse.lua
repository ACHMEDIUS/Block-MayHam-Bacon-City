-- Warehouse map configuration

return {
    name = "Warehouse",
    description = "Close-quarters combat in an abandoned warehouse",

    settings = {
        max_players = 16,
        recommended_mode = "deathmatch",
        theme = "industrial",
    },

    spawn_points = {
        -- Red team spawns
        red = {
            { x = -50.0, y = 0.0, z = -50.0 },
            { x = -45.0, y = 0.0, z = -50.0 },
            { x = -50.0, y = 0.0, z = -45.0 },
            { x = -55.0, y = 0.0, z = -50.0 },
        },
        -- Blue team spawns
        blue = {
            { x = 50.0, y = 0.0, z = 50.0 },
            { x = 45.0, y = 0.0, z = 50.0 },
            { x = 50.0, y = 0.0, z = 45.0 },
            { x = 55.0, y = 0.0, z = 50.0 },
        },
        -- FFA spawns
        ffa = {
            { x = 0.0, y = 0.0, z = 0.0 },
            { x = 20.0, y = 0.0, z = 20.0 },
            { x = -20.0, y = 0.0, z = 20.0 },
            { x = 20.0, y = 0.0, z = -20.0 },
            { x = -20.0, y = 0.0, z = -20.0 },
        }
    },

    weapon_spawns = {
        { weapon = "rifle", x = 0.0, y = 1.0, z = 30.0 },
        { weapon = "shotgun", x = 30.0, y = 1.0, z = 0.0 },
        { weapon = "pistol", x = -30.0, y = 1.0, z = 0.0 },
    },

    on_load = function(map)
        print("Loading map: " .. map.name)
    end,

    on_unload = function(map)
        print("Unloading map: " .. map.name)
    end
}
