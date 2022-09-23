/*use crate::api_shareables::SharedPhysics2D;

use crate::api_shareables::*;
use mlua::prelude::*;

pub fn register_physics2d_api(share_physics: SharedPhysics2D, lua: &Lua) {
	println!("Registering API: Rapier 2D-f64");

	let phys = share_physics.clone();
    let fn_add_static_rect_name = lua.create_function( move |_, (name, x, y, width, height): (String, f64, f64, f64, f64)| {
        phys.borrow_mut().add_static_rect(name, x, y, width, height);
		Ok(())
    }).unwrap();
    let _ = lua.globals().set("phys_add_static_rect", fn_add_static_rect_name);

	let phys = share_physics.clone();
    let fn_add_dynamic_rect_name = lua.create_function( move |_, (name, x, y, width, height): (String, f64, f64, f64, f64)| {
        phys.borrow_mut().add_dynamic_rect(name, x, y, width, height);
		Ok(())
    }).unwrap();
    let _ = lua.globals().set("phys_add_dynamic_rect", fn_add_dynamic_rect_name);

	let phys = share_physics.clone();
    let fn_add_kinematic_body_name = lua.create_function( move |_, (name, x, y, width, height): (String, f64, f64, f64, f64)| {
        phys.borrow_mut().add_kinematic_body_rect(name, x, y, width, height);
		Ok(())
    }).unwrap();
    let _ = lua.globals().set("phys_add_kinematic_body", fn_add_kinematic_body_name);

	let phys = share_physics.clone();
    let fn_add_dynamic_rect_name = lua.create_function( move |_, (dx, dy): (f64, f64)| {
        phys.borrow_mut().set_gravity(dx, dy);
		Ok(())
    }).unwrap();
    let _ = lua.globals().set("phys_set_gravity", fn_add_dynamic_rect_name);
}*/