function _conf()
   set_resolution(960, 540) 
   set_windowed()
end

function _init()
	local random = math.random
	local randomseed = math.randomseed
    randomseed(os.time())

	local time_before = timestamp()

	for x = 0, draw_width(), 1 do
		for y = 0, draw_height(), 1 do
			pset(x, y, hsv(random() * 360, random(), random()))
		end
	end

	local time_after = timestamp()

    print("Noise draw time: " .. (time_after - time_before) .. " seconds!")
end

function _update(delta)
    
end

function _draw()

end