require "lua.std.math"

CONTROLS = {}
CONTROLS.JUMP = 0
CONTROLS.LEFT = 1
CONTROLS.RIGHT = 2

gravity = 100.0

TILE_SIZE = 16.0

level = {}
level[ 1] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
level[ 2] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
level[ 3] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
level[ 4] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
level[ 5] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0}
level[ 6] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
level[ 7] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
level[ 8] = {0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
level[ 9] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
level[10] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
level[11] = {0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0}
level[12] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}


function _conf()
	set_window_title("Platformer Example")
   	set_resolution(384, 216) 
   	set_fullscreen()
    
end

function _init()
	set_key_bind(CONTROLS.JUMP,  "w")
	set_key_bind(CONTROLS.LEFT,  "a")
	set_key_bind(CONTROLS.RIGHT, "d")

    init_level()
end

function _update(delta)
    if global_player ~= nil then
        player_input(global_player, delta)
        body_resolve_level_collision(global_player, level, delta)
    end
end

function init_level()
    -- Go over level tables as a 2D array and spawn entities
    for ty=1, #level, 1 do
        for tx=1, #level[ty], 1 do
            if level[ty][tx] == 9 then
                global_player = spawn_player(tx, ty)
            end
        end
    end
end

function spawn_player(tile_x, tile_y)
    local player = {}

    player.x = tile_x * TILE_SIZE
    player.y = tile_y * TILE_SIZE

    player.width  = TILE_SIZE - 1.0
    player.height = TILE_SIZE - 1.0

    player.dx = 0.0
    player.dy = 0.0

    return player
end

function player_input(player, delta)
    local target_dx, target_dy = 0.0, 0.0

    player.dy = player.dy + (gravity * delta)

    if is_control_down(CONTROLS.LEFT) then
        target_dx = target_dx - 1.0
    end

    if is_control_down(CONTROLS.RIGHT) then
        target_dx = target_dx + 1.0
    end

    if is_control_pressed(CONTROLS.JUMP) then
        target_dy = -1000.0
    end

    target_dx, target_dy = target_dx * 64.0, target_dy * 64.0

    player.dx = lerp(player.dx, target_dx, 5.0 * delta)
end

-- All bodies are the same size as TILE_SIZE for this example. This greatly simplifies collisions
function body_resolve_level_collision(body, level, delta)
    local floor = math.floor
    local next_x = body.x + (body.dx * delta)
    local next_y = body.y + (body.dy * delta)
    
    local tile_topleft = level[floor(next_x / TILE_SIZE)][floor(body.y / TILE_SIZE)]
    local tile_bottomleft = level[floor(next_x / TILE_SIZE)][floor((body.y - (TILE_SIZE - 1.0)) / TILE_SIZE)]
    local tile_topright = level[floor((body.x - (TILE_SIZE - 1.0)) / TILE_SIZE)][floor(body.y / TILE_SIZE)]
    local tile_bottomright = level[floor((body.x - (TILE_SIZE - 1.0)) / TILE_SIZE)][floor((body.y - (TILE_SIZE - 1.0)) / TILE_SIZE)]

    if body.dx < 0.0 then
        if (tile_topleft == 1 or tile_topleft == 2) or (tile_bottomleft == 1 or tile_bottomleft == 2) then
            next_x = floor((next_x + (TILE_SIZE + 1.0) / TILE_SIZE)) * TILE_SIZE
            body.dx = 0.0
        end
    else
        if (tile_topright == 1 or tile_topright == 2) or (tile_bottomright == 1 or tile_bottomright == 2) then
            next_x = floor(next_x / TILE_SIZE) * TILE_SIZE
            body.dx = 0.0
        end
    end

    body.x = next_x
    body.y = next_y
end

function _draw()
    clear_color(rgb(0, 64, 96))

    for ty=1, #level, 1 do
        for tx=1, #level[ty], 1 do
            if level[ty][tx] == 1 then
                prectangle(true, tx * TILE_SIZE, ty * TILE_SIZE, TILE_SIZE, TILE_SIZE, rgb(128, 32, 32))
            end

            if level[ty][tx] == 2 then
                prectangle(true, tx * TILE_SIZE, ty * TILE_SIZE, TILE_SIZE, TILE_SIZE, rgb(32, 128, 32))
            end
        end
    end

    prectangle(true, global_player.x, global_player.y, global_player.width, global_player.height, rgb(0, 255, 0))
end