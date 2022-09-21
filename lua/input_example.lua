walker = { x = 128.0, y = 128.0, w = 16.0, h = 16.0, speed = 32.0, last_dx = 0.0, last_dy = 0.0 }

bullets = {}

CONTROLS = {}
CONTROLS.UP = 0
CONTROLS.DOWN = 1
CONTROLS.LEFT = 2
CONTROLS.RIGHT = 3
CONTROLS.FIRE = 4

function _conf()
	set_window_title("Walking Around")
   	set_resolution(256, 256) 
   	set_fullscreen()

	set_key_bind(CONTROLS.UP, 	 "w")
	set_key_bind(CONTROLS.DOWN,  "s")
	set_key_bind(CONTROLS.LEFT,  "a")
	set_key_bind(CONTROLS.RIGHT, "d")
	set_key_bind(CONTROLS.FIRE,  "space")
end

function _init()

end

function _update(delta)
    local dx, dy = 0.0, 0.0
	local walker = walker
	local controls = CONTROLS

	if is_control_down(controls.UP) then
		dy = dy - 1.0
	end

	if is_control_down(controls.DOWN) then
		dy = dy + 1.0
	end

	if is_control_down(controls.LEFT) then
		dx = dx - 1.0
	end

	if is_control_down(controls.RIGHT) then
		dx = dx + 1.0
	end

	if dx > 0.0 or dx < 0.0 then
		walker.last_dx = dx
	end
	
	if dy > 0.0 or dy < 0.0 then
		walker.last_dy = dy
	end

	if is_mouse_button_pressed(0) then
		local bullet = {x = walker.x, y = walker.y, dx = mouse_x() - walker.x, dy = mouse_y() - walker.y}
		table.insert(bullets, bullet)
	end

	dx, dy = dx * walker.speed * delta, dy * walker.speed * delta
	walker.x, walker.y = walker.x + dx, walker.y + dy

	for i,v in ipairs(bullets) do
		v.x = v.x + v.dx * delta
		v.y = v.y + v.dy * delta
	end
end

function _draw()
	clear_color(rgb(128, 64, 128))
	prectangle(true, walker.x, walker.y, walker.w, walker.h, rgb(0, 255, 0))

	for i,v in ipairs(bullets) do
		pcircle(true, v.x, v.y, 2.0, rgb(255, 255, 0))
	end

	pcircle(false, mouse_x(), mouse_y(), 8.0, rgb(255, 0, 0))
	pline(mouse_x(), mouse_y() - 10.0, mouse_x(), mouse_y() + 10.0, rgb(255, 0, 0))
	pline(mouse_x() - 10.0, mouse_y(), mouse_x() + 10.0, mouse_y(), rgb(255, 0, 0))
end