-- Deathmatch game mode configuration

return {
    name = "Deathmatch",
    description = "Free-for-all combat. First to reach the kill limit wins!",

    settings = {
        kill_limit = 25,
        time_limit = 600,  -- 10 minutes in seconds
        respawn_time = 3.0,
        friendly_fire = false,
        allow_teams = false,
    },

    -- Game mode lifecycle callbacks
    on_start = function(game)
        print("Deathmatch started!")
        game.scores = {}
    end,

    on_player_kill = function(game, killer, victim)
        if not game.scores[killer.id] then
            game.scores[killer.id] = 0
        end
        game.scores[killer.id] = game.scores[killer.id] + 1

        print("Player " .. killer.id .. " killed player " .. victim.id)
        print("Score: " .. game.scores[killer.id])

        -- Check win condition
        if game.scores[killer.id] >= game.settings.kill_limit then
            game:end_match(killer)
        end
    end,

    on_player_death = function(game, player)
        print("Player " .. player.id .. " died. Respawning in " .. game.settings.respawn_time .. " seconds")
    end,

    on_end = function(game, winner)
        print("Deathmatch ended! Winner: Player " .. winner.id)
    end
}
