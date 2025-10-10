-- Team Deathmatch game mode configuration

return {
    name = "Team Deathmatch",
    description = "Red vs Blue team combat. First team to reach the kill limit wins!",

    settings = {
        kill_limit = 50,
        time_limit = 900,  -- 15 minutes in seconds
        respawn_time = 5.0,
        friendly_fire = false,
        allow_teams = true,
        team_balance = true,  -- Auto-balance teams
    },

    teams = {
        red = { name = "Red Team", spawn_points = {}, score = 0 },
        blue = { name = "Blue Team", spawn_points = {}, score = 0 }
    },

    on_start = function(game)
        print("Team Deathmatch started!")
        game.teams.red.score = 0
        game.teams.blue.score = 0
    end,

    on_player_kill = function(game, killer, victim)
        local killer_team = killer.team
        local victim_team = victim.team

        -- Don't count friendly fire
        if killer_team == victim_team then
            print("Friendly fire! No points awarded.")
            return
        end

        -- Award point to killer's team
        game.teams[killer_team].score = game.teams[killer_team].score + 1

        print(killer_team .. " team: " .. game.teams[killer_team].score .. " kills")

        -- Check win condition
        if game.teams[killer_team].score >= game.settings.kill_limit then
            game:end_match(killer_team)
        end
    end,

    on_player_death = function(game, player)
        print("Player " .. player.id .. " (" .. player.team .. ") died. Respawning in " .. game.settings.respawn_time .. " seconds")
    end,

    on_end = function(game, winning_team)
        print("Team Deathmatch ended! " .. game.teams[winning_team].name .. " wins!")
        print("Final score - Red: " .. game.teams.red.score .. ", Blue: " .. game.teams.blue.score)
    end
}
