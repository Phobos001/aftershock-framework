use crate::api_shareables::SharedPhysics2D;

use crate::api_shareables::*;
use mlua::prelude::*;

pub fn register_physics2d_api(share_physics: SharedPhysics2D, lua: &Lua) {
	let phys = share_physics.clone();
    let fn_add_static_rect = lua.create_function( move |_, (x, y, width, height): (f64, f64, f64, f64)| {
        phys.borrow_mut().add_static_rect(x, y, width, height);
		Ok(())
    }).unwrap();
    let _ = lua.globals().set("phys_add_static_rect", fn_add_static_rect);

	let phys = share_physics.clone();
    let fn_add_static_rect_name = lua.create_function( move |_, (name, x, y, width, height): (String, f64, f64, f64, f64)| {
        phys.borrow_mut().add_static_rect_handle(name, x, y, width, height);
		Ok(())
    }).unwrap();
    let _ = lua.globals().set("phys_add_static_rect_name", fn_add_static_rect_name);

	let phys = share_physics.clone();
    let fn_add_dynamic_rect = lua.create_function( move |_, (x, y, width, height): (f64, f64, f64, f64)| {
        phys.borrow_mut().add_static_rect(x, y, width, height);
		Ok(())
    }).unwrap();
    let _ = lua.globals().set("phys_add_dynamic_rect", fn_add_dynamic_rect);

	let phys = share_physics.clone();
    let fn_add_dynamic_rect_name = lua.create_function( move |_, (name, x, y, width, height): (String, f64, f64, f64, f64)| {
        phys.borrow_mut().add_static_rect_handle(name, x, y, width, height);
		Ok(())
    }).unwrap();
    let _ = lua.globals().set("phys_add_dynamic_rect_name", fn_add_dynamic_rect_name);
}