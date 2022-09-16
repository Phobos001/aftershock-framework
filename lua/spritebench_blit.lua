-- Spritebench / BLIT
-- 
-- Blitting is extremely fast; it copies images directly into the buffer (as long as it fits inside of it), 
-- but it has no alpha clipping or any tint/opacity modification like the pixel (p/i) functions
--
-- Useful for lots of images that don't need to rotate.
-- Drawing blits into a seperate buffer and then using a pixel function to draw them on screen could be more performant than
-- drawing each pixel image individually, but it would use more memory

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

		if v.x < 0.0 then v.dx = -v.dx v.x = 0.0 end
		if v.y < 0.0 then v.dy = -v.dy v.y = 0.0 end
		if v.x + 16.0 > screen_width  then v.dx = -v.dx v.x = screen_width - 16.0  end
		if v.y + 16.0 > screen_height then v.dy = -v.dy v.y = screen_height - 16.0 end
	end

	local time_after = timestamp()
	update_time_ms = math.ceil((time_after - time_before) * 10000.0) / 10.0

	
end

function _draw()
	local blit = blit
    
    local time_before = timestamp()
	
	clear_color(rgb(32, 0, 32, 255))

	for i = 1, #balls, 1 do
		blit("ball_sprite", balls[i].x, balls[i].y)
	end

    
    local time_after = timestamp()
	draw_time_ms = math.ceil((time_after - time_before) * 10000.0) / 10.0

	

	pprint(font_tiny, "Update time  : " .. update_time_ms .. "ms", 0, 0)
    pprint(font_tiny, "Draw time    : " .. draw_time_ms .. "ms", 0, 6)
	pprint(font_tiny, "Total balls  : " .. #balls, 0, 12)

	
end