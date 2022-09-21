function _conf()
	set_window_title("Da-da, Da-duuunn!")
   	set_resolution(256, 256) 
   	set_windowed()
end

function _init()
	load_sound("core/sounds/boot.wav", "boot")

	play_sound("boot")
end

function _update(delta)
    
end

function _draw()

end