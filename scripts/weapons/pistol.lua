-- Pistol weapon configuration

return {
    name = "Pistol",
    damage = 15.0,
    fire_rate = 0.25,
    ammo = 12,
    max_ammo = 12,
    reload_time = 1.5,
    accuracy = 0.85,
    recoil = 0.03,
    range = 50.0,

    on_fire = function(weapon, player)
        print("Pistol fired by player " .. player.id)
    end,

    on_reload = function(weapon)
        print("Reloading pistol...")
    end,

    on_hit = function(weapon, target, damage)
        print("Hit target for " .. damage .. " damage")
    end
}
