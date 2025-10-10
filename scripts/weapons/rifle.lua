-- Assault Rifle weapon configuration

return {
    name = "Assault Rifle",
    damage = 25.0,
    fire_rate = 0.1,  -- seconds between shots
    ammo = 30,
    max_ammo = 30,
    reload_time = 2.0,
    accuracy = 0.95,
    recoil = 0.05,
    range = 100.0,

    -- Callbacks for weapon behavior
    on_fire = function(weapon, player)
        -- Custom fire logic can go here
        print("Rifle fired by player " .. player.id)
    end,

    on_reload = function(weapon)
        print("Reloading rifle...")
    end,

    on_hit = function(weapon, target, damage)
        print("Hit target for " .. damage .. " damage")
    end
}
