mod controls;

//mod rhai;
mod lua;

// Math
mod math;
mod vector2;
mod matrix3;

// Software Rendering
mod color;
mod font;
mod rasterizer;
mod partitioned_rasterizer;

// Physics
mod rapier2d_wrap;

// Lua API
mod api_shareables;
mod api_audio;
mod api_color;
mod api_display;
mod api_drawing;
mod api_font;
mod api_image;
mod api_input;
mod api_physics;
mod api_profiling;

mod error_data;

const CURSOR_IMAGE: &[u8] = include_bytes!("cursor.png");

fn get_cursor_img() -> Rasterizer {
    use rgb::ComponentBytes;
    let decode_result = lodepng::decode32(CURSOR_IMAGE);
    if decode_result.is_ok() {
        let img = decode_result.unwrap();
        let mut r = Rasterizer::new(img.width, img.height);
        r.color = img.buffer.as_bytes().to_vec();
        r
    } else {
        panic!("ERROR - MAIN: CURSOR_IMAGE cannot be decoded!");
    }
}

use crate::color::Color;
use crate::font::Font;
use crate::lua::LuaScript;
use crate::rasterizer::Rasterizer;

use sdl2::event::Event;
use sdl2::pixels::{PixelFormatEnum};
use sdl2::video::FullscreenType;


use std::time::Instant;

const TITLE: &str = "Aftershock Framework";
const VERSION: &str = "v. 0.1.0";

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EngineVideoMode {
    Exclusive,
    Fullscreen,
    Windowed,
}

pub struct TimeData {
    pub realtime: f64,
    pub tics: u64,
    pub dt: f64,

    dt_before: Instant,
}

pub struct VideoData {
    pub screen_resolution: (usize, usize),
    pub window_title:   String,
    pub mode: EngineVideoMode,
    pub stretch_fill: bool,
}

pub struct AftershockEngine {
    pub lua_global: LuaScript,
    pub video: VideoData,
    pub time: TimeData,
}



impl TimeData {
    pub fn update(&mut self) {
        let now = Instant::now();
        let now_s = (now.elapsed().as_secs() as f64) + (now.elapsed().subsec_nanos() as f64 * 1.0e-9);
        let before_s = (self.dt_before.elapsed().as_secs() as f64) + (self.dt_before.elapsed().subsec_nanos() as f64 * 1.0e-9);
        self.dt = before_s - now_s;
        
        self.dt_before = Instant::now();
        self.realtime += self.dt;
    }
}

impl AftershockEngine {
    pub fn new(main_lua: String) -> Result<AftershockEngine, String> {

        let screen_resolution: (usize, usize) =  (960, 540);

        if main_lua.is_empty() {
            return Err("ERROR: Game not found! Use \"--game <game_path>.lua\" to load your game!\nFor example, \"--game src/main.lua\" or \"--game tools/level_editor.lua\"".to_string());
        }

        let lua_global_result =  LuaScript::new(main_lua);
        if lua_global_result.is_ok() {
            Ok(AftershockEngine {
                lua_global: lua_global_result.unwrap(),
    
                time: TimeData {
                    dt: 0.0,
                    dt_before: Instant::now(),
                    realtime: 0.0,
                    tics: 0,
                },
    
                video: VideoData {
                    screen_resolution,
                    window_title: "".to_string(),
                    mode: EngineVideoMode::Windowed,
                    stretch_fill: false,
                },
            })
        } else {
            Err(lua_global_result.err().unwrap())
        }

        
	}
}

pub fn main() {
    const DEFAULT_WIDTH: u32 = 384;
    const DEFAULT_HEIGHT: u32 = 216;
    
    let mut hardware_accelerated: bool = false;
    let args: Vec<String> = std::env::args().collect();

    let mut script: String = String::from("");
    let mut loaded_main_lua: bool = false;

    let mut max_update_hz: f64 = 1.0 / 144.0;
    let mut max_draw_hz: f64 = 1.0 / 144.0;

    let mut last_width: usize = DEFAULT_WIDTH as usize;
    let mut last_height: usize = DEFAULT_HEIGHT as usize;

    let mut lua_error: Option<String> = None;

    for i in 0..args.len()-1 {
        match args[i].as_str() {
            "--game" => {
                let lua_main_result = std::fs::read_to_string(args[i+1].as_str());
                if lua_main_result.is_ok() {
                    script = lua_main_result.unwrap();
                    loaded_main_lua = true;
                }
            },
            "--draw-hz" => {
                let parsed = args[i+1].parse::<f64>();
                if parsed.is_ok() {
                    let hz = parsed.unwrap();
                    max_draw_hz = 1.0 / hz;
                }
            },
            "--update-hz" => {
                let parsed = args[i+1].parse::<f64>();
                if parsed.is_ok() {
                    let hz = parsed.unwrap();
                    max_update_hz = 1.0 / hz;
                }
            }
            "--hardware-accelerated" => { hardware_accelerated = true; },
            _ => {}
        }
    }

    if !loaded_main_lua {
        lua_error = Some("ERROR - LUA: Game not found!".to_string());
    }
    
    let engine_result = AftershockEngine::new(script);

    let engine_option: Option<AftershockEngine> = if engine_result.is_ok() { 
        Some(engine_result.unwrap())
    } else {
        lua_error = Some(engine_result.err().unwrap()); None
    };

    println!("\n===== {} {} =====\n",TITLE, VERSION);

    // Init SDL and surface texture
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    println!("SDL Version: {}", sdl2::version::version());

    let window = video_subsystem
        .window(TITLE, DEFAULT_WIDTH, DEFAULT_HEIGHT)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

        
    let mut canvas = if hardware_accelerated { 
        println!("Hardware Canvas");
        window.into_canvas().present_vsync().build().map_err(|e| e.to_string()).unwrap()
    } else {
        println!("Software Canvas");
        window.into_canvas().software().present_vsync().build().map_err(|e| e.to_string()).unwrap()
    };

    let texture_creator = canvas.texture_creator();

    // This is what we update our buffers to
    let mut screentex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32, DEFAULT_WIDTH, DEFAULT_HEIGHT)
    .map_err(|e| e.to_string()).unwrap();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let _ = canvas.set_logical_size(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    let _ = canvas.set_integer_scale(true);
    let _ = canvas.window_mut().set_minimum_size(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    
    // Run the engine if there are no errors!

    if engine_option.is_some() {
        let mut engine = engine_option.unwrap();

        engine.lua_global.video_data.borrow_mut().screen_resolution = engine.video.screen_resolution;
        engine.lua_global.video_data.borrow_mut().stretch_fill = engine.video.stretch_fill;
        engine.lua_global.video_data.borrow_mut().mode = engine.video.mode;

        engine.lua_global.controls.borrow_mut().update_mouse_boundries(DEFAULT_WIDTH as f64, DEFAULT_HEIGHT as f64);

        let mut game_maxfps_timer: f64 = 0.0;
        let mut draw_maxfps_timer: f64 = 0.0;

        println!("Running Game Loop!");

        let conf_error = engine.lua_global.conf();
        if conf_error.is_err() {
            lua_error = Some(format!("Runtime Error: Lua: {}", conf_error.err().unwrap()));
        }

        let init_error = engine.lua_global.init();
        if init_error.is_err() {
            lua_error = Some(format!("Runtime Error: Lua: {}", init_error.err().unwrap()));
        }

        // We need to monitor the real change in time between updates
        let mut last_update_time: f64 = 0.0;

        let spr_cursor: Rasterizer = get_cursor_img();

        'gameloop: loop {
            if lua_error.is_some() { break 'gameloop; }

            sdl_context.mouse().show_cursor(false);
            //sdl_context.mouse().set_relative_mouse_mode(true);
            

            engine.time.update();
            game_maxfps_timer -= engine.time.dt;
            draw_maxfps_timer -= engine.time.dt;



            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'gameloop
                    },
                    _ => {}
                }
            }

            // Check for altered window properties
            {
                // Check for resize
                let (rst_width, rst_height) = (engine.lua_global.rasterizer.borrow().rasterizer.width, engine.lua_global.rasterizer.borrow().rasterizer.height);
                if  rst_width != last_width || rst_height != last_height {
                    engine.video.screen_resolution = (rst_width, rst_height);

                    canvas.clear();

                    screentex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32, rst_width as u32, rst_height as u32)
                    .map_err(|e| e.to_string()).unwrap();

                    let _ = canvas.window_mut().set_size(rst_width as u32, rst_height as u32);

                    if !engine.video.stretch_fill {
                        let _ = canvas.set_logical_size(rst_width as u32, rst_height as u32);
                        let _ = canvas.set_integer_scale(true);
                        let _ = canvas.window_mut().set_minimum_size(rst_width as u32, rst_height as u32);
                    }

                    engine.lua_global.controls.borrow_mut().update_mouse_boundries(rst_width as f64, rst_height as f64);
                    
                    last_width = rst_width;
                    last_height = rst_height;
                }

                // Check for window mode
                let lua_video_mode: EngineVideoMode = engine.lua_global.video_data.borrow_mut().mode;
                if engine.video.mode != lua_video_mode {
                    match lua_video_mode {
                        EngineVideoMode::Fullscreen => {
                            let _ = canvas.window_mut().set_bordered(false);
                            let _ = canvas.window_mut().maximize();
                            let _ = canvas.window_mut().set_fullscreen(FullscreenType::Desktop);
                        },
                        EngineVideoMode::Windowed => {
                            let _ = canvas.window_mut().set_bordered(true);
                            let _ = canvas.window_mut().set_size(last_width as u32, last_height as u32);
                            let _ = canvas.window_mut().set_fullscreen(FullscreenType::Off);
                        },
                        EngineVideoMode::Exclusive => {
                            let _ = canvas.window_mut().set_fullscreen(FullscreenType::True);
                        },
                    }
                }

                engine.video.mode = lua_video_mode;

                // Check for window title
                if engine.video.window_title != engine.lua_global.video_data.borrow().window_title {
                    let _ = canvas.window_mut().set_title(engine.lua_global.video_data.borrow().window_title.as_str());
                }
            }

            if game_maxfps_timer <= 0.0 {
                let update_dt = engine.time.realtime - last_update_time;
                last_update_time = engine.time.realtime;

                // == GAME ==
                
                engine.lua_global.controls.borrow_mut().update_controls(event_pump.mouse_state(), event_pump.keyboard_state());
                //sdl_context.mouse().warp_mouse_in_window(canvas.window(), last_width as i32 / 2, last_height as i32 / 2);
                
                // Run Lua Update
                let update_error = engine.lua_global.update(update_dt);
                if update_error.is_err() {
                    lua_error = Some(format!("Runtime Error: Lua: {}", update_error.err().unwrap()));
                }
                
                engine.time.tics += 1;
                
                game_maxfps_timer = max_update_hz;
            }

            if draw_maxfps_timer <= 0.0 {

                let draw_error = engine.lua_global.draw();
                if draw_error.is_err() {
                    lua_error = Some(format!("Runtime Error: Lua: {}", draw_error.err().unwrap()));
                }

                use crate::rasterizer::DrawMode;
/*                 if engine.lua_global.controls.borrow().mouse_active {
                    let mut rst = engine.lua_global.rasterizer.borrow_mut();

                    rst.set_draw_mode(DrawMode::Alpha);
                    rst.set_tint(Color::black());
                    rst.set_opacity(127);
                    rst.pimg(&spr_cursor,
                        engine.lua_global.controls.borrow().mouse.x as i64 + 1,
                        engine.lua_global.controls.borrow().mouse.y as i64 + 1
                    );

                    rst.set_draw_mode(DrawMode::Opaque);
                    rst.set_tint(Color::white());
                    rst.set_opacity(255);
                    rst.pimg(&spr_cursor,
                        engine.lua_global.controls.borrow().mouse.x as i64,
                        engine.lua_global.controls.borrow().mouse.y as i64
                    );
                } */

                // Present to screen
                let _ = screentex.update(None, &engine.lua_global.rasterizer.borrow().rasterizer.color, (engine.lua_global.rasterizer.borrow().rasterizer.width * 4) as usize);
                let _ = canvas.copy(&screentex, None, None);
                canvas.present();

                draw_maxfps_timer = max_draw_hz;
            }

            std::thread::sleep(std::time::Duration::from_micros(10));
        }
    }

    if lua_error.is_some() {
        canvas.clear();

        let error_text = lua_error.unwrap().to_uppercase();

        screentex = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32, 512, 512)
        .map_err(|e| e.to_string()).unwrap();


        use crate::color::*;

        let error_bg_img: Rasterizer = error_data::get_error_bg();
        //let error_text_img: Rasterizer = error_data::raster_text_to_image(512, 512, error_text);
        let tiny_font_img = error_data::get_tiny_font();

        let tiny_font: Font = Font {
            fontimg: tiny_font_img,
            glyph_height: 10,
            glyph_width: 10,
            glyph_spacing: 0,
            glyphidx_sizes: Vec::new(),
            glyphidx: "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!?/\\@#$%^&*()[]_-+=\"';:".to_string().chars().collect()
        };

        let mut error_rast: Rasterizer = Rasterizer::new(512, 512);
        
        for i in 0..error_rast.height {
            error_rast.pline(0, i as i64, error_rast.width as i64, i as i64, Color::hsv((i as f64) * 0.1, 1.0, 0.35));
        }

        error_rast.pprint(&tiny_font, error_text, 8, 8, 5, Some(450));



        let _ = canvas.set_logical_size(512, 512);
        let _ = canvas.set_integer_scale(false);

        let _ = canvas.window_mut().set_fullscreen(FullscreenType::Off);
        let _ = canvas.window_mut().set_minimum_size(512, 512);
        let _ = canvas.window_mut().set_size(512 as u32, 512 as u32);

        let _ = screentex.update(None, &error_rast.color, (error_rast.width * 4) as usize);
        let _ = canvas.copy(&screentex, None, None);
        canvas.present();

        'errorloop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'errorloop
                    },
                    _ => {}
                }
            }

            canvas.clear();

            let _ = screentex.update(None, &error_rast.color, (error_rast.width * 4) as usize);
            let _ = canvas.copy(&screentex, None, None);
            canvas.present();

            std::thread::sleep(std::time::Duration::from_micros(10));
        }
    }

    println!("Shutting down...");
}