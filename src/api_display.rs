use std::rc::Rc;
use std::cell::RefCell;

use crate::VideoData;
use crate::EngineVideoMode;

use mlua::prelude::*;

use crate::api_shareables::*;

pub fn register_display_api(rasterizer: SharedRasterizer, video_data: SharedVideoData, lua: &Lua) {

    let rst = rasterizer.clone();
    let fn_display_set_resolution = lua.create_function(move |_, (width, height) :(f64, f64)| {
        rst.borrow_mut().resize(width as usize, height as usize);
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("set_resolution", fn_display_set_resolution);

    let vid = video_data.clone();
    let fn_display_set_windowed = lua.create_function(move |_, ()| {
        vid.borrow_mut().mode = EngineVideoMode::Windowed;
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("set_windowed", fn_display_set_windowed);

    let vid = video_data.clone();
    let fn_display_set_fullscreen = lua.create_function(move |_, ()| {
        vid.borrow_mut().mode = EngineVideoMode::Fullscreen;
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("set_fullscreen", fn_display_set_fullscreen);

    let vid = video_data.clone();
    let fn_display_set_exclusive = lua.create_function(move |_, ()| {
        vid.borrow_mut().mode = EngineVideoMode::Exclusive;
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("set_exclusive", fn_display_set_exclusive);

    let rst = rasterizer.clone();
    let fn_display_width = lua.create_function(move |_, ()| {
        Ok(rst.borrow().width)
    }).unwrap();
    let _ = lua.globals().set("draw_width", fn_display_width);

    let rst = rasterizer.clone();
    let fn_display_height = lua.create_function(move |_, ()| {
        Ok(rst.borrow().height)
    }).unwrap();
    let _ = lua.globals().set("draw_height", fn_display_height);
}