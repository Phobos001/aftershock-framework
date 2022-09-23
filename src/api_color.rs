use crate::color::*;
use mlua::prelude::*;

pub fn register_color(lua: &Lua) {
    println!("Registering API: Color");

    // RGB //
    let rgb_constructor = lua.create_function(|_, (r, g, b): (f64, f64, f64)| {
        let r: u8 = f64::clamp(r, 0.0, 255.0) as u8;
        let g: u8 = f64::clamp(g, 0.0, 255.0) as u8;
        let b: u8 = f64::clamp(b, 0.0, 255.0) as u8;
        let a: u8 = 255;
        Ok(Color::new(r, g, b, a))
    }).unwrap();
    let _ = lua.globals().set("rgb", rgb_constructor);

    // RGBA //
    let rgba_constructor = lua.create_function(|_, (r, g, b, a): (f64, f64, f64, f64)| {
        let r: u8 = f64::clamp(r, 0.0, 255.0) as u8;
        let g: u8 = f64::clamp(g, 0.0, 255.0) as u8;
        let b: u8 = f64::clamp(b, 0.0, 255.0) as u8;
        let a: u8 = f64::clamp(a, 0.0, 255.0) as u8;
        Ok(Color::new(r, g, b, a))
    }).unwrap();
    let _ = lua.globals().set("rgba", rgba_constructor);

    // HSV //
    let hsv_constructor = lua.create_function(|_, (hue, saturation, value): (f64, f64, f64)| {
        Ok(Color::hsv(hue, saturation, value))
    }).unwrap();
    let _ = lua.globals().set("hsv", hsv_constructor);
}