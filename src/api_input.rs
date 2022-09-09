use crate::api_shareables::*;
use mlua::prelude::*;

pub fn register_input_api(control_data: SharedControlData, lua: &Lua) {

    let input = control_data.clone();
    let fn_mouse_x = lua.create_function( move |_, ()| {
        Ok(input.borrow().mouse.x)
    }).unwrap();
    let _ = lua.globals().set("mouse_x", fn_mouse_x);

    let input = control_data.clone();
    let fn_mouse_y = lua.create_function( move |_, ()| {
        Ok(input.borrow().mouse.y)
    }).unwrap();
    let _ = lua.globals().set("mouse_y", fn_mouse_y);
}