-- Shotgun weapon configuration

return {
    name = "Shotgun",
    damage = 60.0,
    fire_rate = 0.8,
    ammo = 8,
    max_ammo = 8,
    reload_time = 3.0,
    accuracy = 0.6,  -- Lower accuracy for spread
    recoil = 0.15,
    range = 25.0,
    pellets = 8,  -- Shotgun-specific: fires multiple pellets

    on_fire = function(weapon, player)
        print("Shotgun fired by player " .. player.id .. " (" .. weapon.pellets .. " pellets)")
    end,

    on_reload = function(weapon)
        print("Reloading shotgun...")
    end,

    on_hit = function(weapon, target, damage)
        print("Hit target for " .. damage .. " damage")
    end
}
