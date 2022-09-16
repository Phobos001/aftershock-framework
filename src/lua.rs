use dashmap::DashMap;
use mlua::prelude::*;

use crate::api_audio::*;
use crate::api_color::*;
use crate::api_display::*;
use crate::api_drawing::*;
use crate::api_font::*;
use crate::api_image::*;
use crate::api_input::*;
use crate::api_profiling::*;

use crate::api_shareables::*;

use crate::controls::ControlData;
use crate::VideoData;
use crate::EngineVideoMode;

use crate::rasterizer::Rasterizer;
use crate::partitioned_rasterizer::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;

pub struct LuaScript {
    pub window_title:   String,
    pub video_data:     SharedVideoData,

    pub lua:            mlua::Lua,

    pub controls:       SharedControlData,
    pub rasterizer:     SharedRasterizer,
    pub audio:          SharedAudio,

    pub assets_sfx:     SharedAudioWav,
    pub assets_mus:     SharedAudioWavStream,
    pub assets_img:     SharedImages,
}

impl LuaScript {
    pub fn new(title: String, script: String) -> Result<LuaScript, String> {
        let lua = Lua::new();

        // Fatal Error if Audio API cannot init.
        // Maybe able to allow continuing if the user doesn't want audio?
        let soloud_result = soloud::Soloud::default();
        if soloud_result.is_err() {
            return Err(format!("ERROR - AUDIO: Soloud failed to initialize! Soloud: {}", soloud_result.err().unwrap()))
        }

        let video_data: SharedVideoData         = Rc::new(RefCell::new(
            VideoData { screen_resolution: (384, 216), stretch_fill: false, mode: EngineVideoMode::Windowed})
        );

        let rasterizer: SharedRasterizer        = Rc::new(RefCell::new(PartitionedRasterizer::new(384, 216, 0)));
        let controls:   SharedControlData       = Rc::new(RefCell::new(ControlData::new()));

        let assets_sfx: SharedAudioWav          = Arc::new(DashMap::new());
        let assets_mus: SharedAudioWavStream    = Arc::new(DashMap::new());
        let assets_img: SharedImages            = Arc::new(DashMap::new());

        let soloud: SharedAudio = Arc::new(soloud_result.unwrap());

        register_audio_api(soloud.clone(), assets_sfx.clone(), assets_mus.clone(), &lua);
        register_color(&lua);
        register_display_api(rasterizer.clone(), video_data.clone(), &lua);
        register_draw_api(assets_img.clone(), rasterizer.clone(), &lua);
        register_input_api(controls.clone(), &lua);
        register_image(assets_img.clone(),&lua);
        register_profiling_api(&lua);
        register_font(&lua);

        
        // Exec is recommended so variables loaded outside callback functions are readied
        let test_file = lua.load(&script).exec();
        if test_file.is_err() {
            let e = test_file.err().unwrap();
            Err(format!("Lua: file failed to load! Error: {}", e))
        } else {
            Ok(LuaScript { window_title: title, video_data, lua, controls, rasterizer, audio: soloud, assets_sfx, assets_mus, assets_img})
        }
    }

    pub fn framebuffer_to_u32(&self) -> Vec<u32> {
        self.rasterizer.borrow_mut().rasterizer.color.chunks_exact(4)
                        .map(|c| (c[0] as u32) << 16 | (c[1] as u32) << 8 | (c[2] as u32) << 0)
                        .collect()
    }

    pub fn conf(&mut self) -> Result<(), LuaError> {
        self.lua.globals().call_function("_conf".to_lua(&self.lua).unwrap(), ())
    }

    pub fn init(&mut self) -> Result<(), LuaError> {
        self.lua.globals().call_function("_init".to_lua(&self.lua).unwrap(), ())
    }

    pub fn update(&mut self, dt: f64) -> Result<(), LuaError> {
        self.lua.globals().call_function("_update".to_lua(&self.lua).unwrap(), dt.to_lua_multi(&self.lua).unwrap())
    }

    pub fn draw(&mut self) -> Result<(), LuaError>{
        self.lua.globals().call_function("_draw".to_lua(&self.lua).unwrap(), ())
    }
}