realtime = 0.0

TINY_FONT_GLYPHIDX = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!?*^&()[]<>-+=/\\\"'`~:;,.%abcdefghijklmnopqrstuvwxyz"

font_tiny = load_font("core/fonts/tiny_font.png", TINY_FONT_GLYPHIDX, 5, 5, -1);

printdt = 0.0

function _conf()
   set_resolution(960, 540) 
   set_fullscreen()
end

function _init()
    math.randomseed(os.time())
    
    race_time = 0.0

    racers = { x = {}, y = {}, id = {}, color = {}}

    racing = true
    has_finished = false

    winners = {}

    local display_height = draw_height()
    for i = 1, display_height, 1 do
        racers.x[i] = 0.0
        racers.y[i] = i
        racers.color[i] = hsv(math.random() * 360.0, 1.0, 1.0)
    end
    print("Built " .. #racers.x .. " racers!")
end

function _update(delta)
    local update_time_before = timestamp()
    local ceil = math.ceil
    realtime = realtime + delta
    race_time = race_time + delta

    printdt = delta

    local height = draw_height()
    local width = draw_width()
    if racing then
        for i = 1, height, 1 do
            racers.x[i] = racers.x[i] + (math.random() * 512.0) * delta
            if racers.x[i] > width then
                table.insert(winners, {id = racers.y[i], at = racers.x[i]})
            end
        end

        if #winners > 0 then
            racing = false
        end
    else
        if not has_finished then
            print("All done!\nWinners: ")
            for i,v in ipairs(winners) do
                print("     * " .. v.id .. " at " .. v.at)
            end
            print("Race time: " .. race_time .. " seconds!")
            has_finished = true
            
        else
            
            _init()
        end
    end

    local ceil = math.ceil
    local update_time_after = timestamp()
    update_time = ceil((update_time_after - update_time_before) * 100000.0) / 100.0
    
end

function _draw()
    local ceil = math.ceil
    
    local draw_time_before = timestamp()

    local height = draw_height()

    clear_color(hsv(realtime * 90.0, 1.0, 0.25))

    
    for i = 1, height, 1 do
        pset(racers.x[i], racers.y[i]-1.0, racers.color[i])
    end
    
    local draw_time_after = timestamp()
    
    local final_draw_time_in_ms = ceil((draw_time_after - draw_time_before) * 100000.0) / 100.0
    pprint(font_tiny, "Draw time: " .. final_draw_time_in_ms .. "ms", 0, 0)
    pprint(font_tiny, "Update time: " .. update_time .. "ms", 0, 6)
    pprint(font_tiny, "dt: " .. printdt, 0, 12)
end