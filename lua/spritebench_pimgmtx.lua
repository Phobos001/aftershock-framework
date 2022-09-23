-- Spritebench / PIMGMTX
-- 
-- Pixel images using the matrix function draw in camera space, using 2D Affine Transformations. They are much slower than their non-transformed counterparts,
-- but allow for panning, zooming, and rotation, both for the camera and for the sprite together.

realtime = 0.0

target_ms = 16.67 -- 60 FPS
update_time_ms = 0.0
draw_time_ms = 0.0

TINY_FONT_GLYPHIDX = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!?*^&()[]<>-+=/\\\"'`~:;,.%abcdefghijklmnopqrstuvwxyz"

font_tiny = load_font("core/fonts/tiny_font.png", TINY_FONT_GLYPHIDX, 5, 5, -1)

balls = {}

-- Using an OOP pattern for right now, might make an ECS version to test differences
function make_ball()
	local random = math.random

	local ball = {}
	ball.x = draw_width() / 2.0
	ball.y = draw_height() / 2.0
	ball.color = hsv(random() * 360.0, 1.0, rand_range(0.5, 1.0))
	ball.opacity = rand_range(0.0, 255.0)
	ball.dx = rand_range(-128.0, 128.0)
	ball.dy = rand_range(-128.0, 128.0)
	ball.size = rand_range(0.1, 1.0)
	return ball
end


function _conf()
    set_resolution(640, 360)
    set_fullscreen()
	set_core_limit(0)
end

function _init()
    load_image("ball_sprite", "core/sprites/ball.png")

end

function rand_range(min, max)
    local random = math.random
    return min + (max - min) * random()
end

function _update(delta)
	local abs = math.abs

	local screen_width = draw_width()
	local screen_height = draw_height()

	local time_before = timestamp()

    realtime = realtime + delta

	if draw_time_ms < target_ms then
		table.insert(balls, make_ball())
	end

	for i, v in ipairs(balls) do
		v.x = v.x + (v.dx * delta)
		v.y = v.y + (v.dy * delta)

		if v.x < 8.0 then v.dx = -v.dx v.x = 8 end
		if v.y < 8.0 then v.dy = -v.dy v.y = 8 end
		if v.x + 8.0 > screen_width  then v.dx = -v.dx v.x = screen_width - 8.0  end
		if v.y + 8.0 > screen_height then v.dy = -v.dy v.y = screen_height - 8.0 end
	end

	local time_after = timestamp()
	update_time_ms = math.ceil((time_after - time_before) * 10000.0) / 10.0

	
end

function _draw()
	local set_tint = set_tint
	local set_opacity = set_opacity
	local pimgmtx = pimgmtx
	local sin = math.sin
    
    local time_before = timestamp()

	local camera_scale = 2.0 + sin(realtime * 0.5) * 1.0
	local camera_rotation = sin(realtime * 0.25) * 0.2

	set_camera_scale(camera_scale, camera_scale)
	set_camera_rotation(camera_rotation)
	update_camera()
	
	set_draw_mode_alpha()
	clear_color(rgb(32, 0, 32, 255))

	for i = 1, #balls, 1 do
		set_tint(balls[i].color)
		set_opacity(balls[i].opacity)
		pimgmtx("ball_sprite", balls[i].x, balls[i].y, 0.0, 1.0, 1.0, 0.5, 0.5)
	end
	set_draw_mode_opaque()
	set_tint(rgb(255, 255, 255))
	set_opacity(255)
	
    
    local time_after = timestamp()
	draw_time_ms = math.ceil((time_after - time_before) * 10000.0) / 10.0

	

	pprint(font_tiny, "Update time  : " .. update_time_ms .. "ms", 0, 0)
    pprint(font_tiny, "Draw time    : " .. draw_time_ms .. "ms", 0, 6)
	pprint(font_tiny, "Total balls  : " .. #balls, 0, 12)

	
end