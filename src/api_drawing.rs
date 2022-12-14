use crate::color::*;
use crate::rasterizer::*;
use crate::font::*;

use mlua::prelude::*;

use crate::api_shareables::*;

pub fn register_draw_api(assets_images: SharedImages, rasterizer: SharedRasterizer, lua: &Lua) {
    println!("Registering API: Drawing");

    let rst = rasterizer.clone();
    let fn_update_camera = lua.create_function(move |_, ()| {
        rst.borrow_mut().update_camera();
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("update_camera", fn_update_camera);

    // Set Core Limit //
    let rst = rasterizer.clone();
    let fn_set_draw_mode_noop = lua.create_function(move |_, cores: f64| {
        if cores > 0.0 {
            let cores: usize = f64::floor(cores) as usize;
            rst.borrow_mut().set_core_limit(cores);
            Ok(())
        } else {
            rst.borrow_mut().set_core_limit(0);
            Ok(())
        }
        
    } ).unwrap();
    let _ = lua.globals().set("set_core_limit", fn_set_draw_mode_noop);

    // Draw Mode: No Operation //
    let rst = rasterizer.clone();
    let fn_set_draw_mode_noop = lua.create_function(move |_, ()| {
        rst.borrow_mut().set_draw_mode(DrawMode::NoOp);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_draw_mode_noop", fn_set_draw_mode_noop);

    // Draw Mode: Opaque //
    let rst = rasterizer.clone();
    let fn_set_draw_mode_opaque = lua.create_function(move |_, ()| {
        rst.borrow_mut().set_draw_mode(DrawMode::Opaque);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_draw_mode_opaque", fn_set_draw_mode_opaque);

    // Draw Mode: Alpha //
    let rst = rasterizer.clone();
    let fn_set_draw_mode_alpha = lua.create_function(move |_, ()| {
        rst.borrow_mut().set_draw_mode(DrawMode::Alpha);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_draw_mode_alpha", fn_set_draw_mode_alpha);

    // Draw Mode: Addition //
    let rst = rasterizer.clone();
    let fn_set_draw_mode_addition = lua.create_function(move |_, ()| {
        rst.borrow_mut().set_draw_mode(DrawMode::Addition);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_draw_mode_addition", fn_set_draw_mode_addition);

    // Draw Mode: Subtract //
    let rst = rasterizer.clone();
    let fn_set_draw_mode_subtraction = lua.create_function(move |_, ()| {
        rst.borrow_mut().set_draw_mode(DrawMode::Subtraction);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_draw_mode_subtraction", fn_set_draw_mode_subtraction);

    // Draw Mode: Multiply //
    let rst = rasterizer.clone();
    let fn_set_draw_mode_multiply = lua.create_function(move |_, ()| {
        rst.borrow_mut().set_draw_mode(DrawMode::Multiply);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_draw_mode_multiply", fn_set_draw_mode_multiply);

    // Draw Mode: Force Tint //
    let rst = rasterizer.clone();
    let fn_set_draw_mode_force_tint = lua.create_function(move |_, ()| {
        rst.borrow_mut().set_draw_mode(DrawMode::ForceTint);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_draw_mode_force_tint", fn_set_draw_mode_force_tint);

    // Clear //
    let rst = rasterizer.clone();
    let fn_clear = lua.create_function(move |_, ()| {
        rst.borrow_mut().clear();
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("clear", fn_clear);

    // Clear Color //
    let rst = rasterizer.clone();
    let fn_clear_color = lua.create_function(move |_, color: Color| {
        rst.borrow_mut().clear_color(color);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("clear_color", fn_clear_color);

    // Set Tint //
    let rst = rasterizer.clone();
    let fn_set_tint = lua.create_function(move |_, color: Color| {
        rst.borrow_mut().set_tint(color);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_tint", fn_set_tint);

    // Set Opacity //
    let rst = rasterizer.clone();
    let fn_set_opacity = lua.create_function(move |_, opacity: f64| {
        let opacity: u8 = f64::clamp(opacity, 0.0, 255.0) as u8;
        rst.borrow_mut().set_opacity(opacity);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_opacity", fn_set_opacity);

    // Set Camera Position //
    let rst = rasterizer.clone();
    let fn_set_camera_position = lua.create_function(move |_, (x, y): (f64, f64)| {
        rst.borrow_mut().set_camera_position(x, y);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_camera_position", fn_set_camera_position);

    // Set Camera Rotation //
    let rst = rasterizer.clone();
    let fn_set_camera_rotation = lua.create_function(move |_, r: f64| {
        rst.borrow_mut().set_camera_rotation(r);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_camera_rotation", fn_set_camera_rotation);

    // Set Camera Scale //
    let rst = rasterizer.clone();
    let fn_set_camera_scale = lua.create_function(move |_, (x, y): (f64, f64)| {
        rst.borrow_mut().set_camera_scale(x, y);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("set_camera_scale", fn_set_camera_scale);

    // blit sprite //
    let rst = rasterizer.clone();
    let imga = assets_images.clone();
    let fn_blit = lua.create_function(move |_, (name, x, y): (String, f64, f64)| {
        let img_result = imga.get(&name);
        if img_result.is_some() {
            rst.borrow_mut().blit(&img_result.unwrap(), x as i64, y as i64);
        }
        
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("blit", fn_blit);

    // pset //
    let rst = rasterizer.clone();
    let fn_pset = lua.create_function(move |_, (x, y, color): (i64, i64, Color)| {
        rst.borrow_mut().pset(x, y, color);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("pset", fn_pset);

    // pline //
    let rst = rasterizer.clone();
    let fn_pline = lua.create_function(move |_, (x0, y0, x1, y1, color): (i64, i64, i64, i64, Color)| {
        rst.borrow_mut().pline(x0, y0, x1, y1, color);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("pline", fn_pline);

    // pcircle //
    let rst = rasterizer.clone();
    let fn_pcircle = lua.create_function(move |_, (filled, xc, yc, r, color): (bool, i64, i64, i64, Color)| {
        rst.borrow_mut().pcircle(filled, xc, yc, r, color);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("pcircle", fn_pcircle);

    // prectangle //
    let rst = rasterizer.clone();
    let fn_prectangle = lua.create_function(move |_, (filled, x, y, w, h, color): (bool, i64, i64, i64, i64, Color)| {
        rst.borrow_mut().prectangle(filled, x, y, w, h, color);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("prectangle", fn_prectangle);

    // ptriangle //
    let rst = rasterizer.clone();
    let fn_ptriangle = lua.create_function(move |_, (filled, x0, y0, x1, y1, x2, y2, color): (bool, i64, i64, i64, i64, i64, i64, Color)| {
        rst.borrow_mut().rasterizer.ptriangle(filled, x0, y0, x1, y1, x2, y2, color);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("ptriangle", fn_ptriangle);

    // pbeizer //
    let rst = rasterizer.clone();
    let fn_pbeizer = lua.create_function(move |_, (x0, y0, x1, y1, mx, my, color): (i64, i64, i64, i64, i64, i64, Color)| {
        rst.borrow_mut().rasterizer.pbeizer(x0, y0, x1, y1, mx, my, color);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("pbeizer", fn_pbeizer);

    // pimg //
    let rst = rasterizer.clone();
    let imga = assets_images.clone();
    let fn_pimg = lua.create_function(move |_, (name, x, y): (String, f64, f64)| {
        //let imga_ref = imga.get();
        let img_result = imga.get(&name);
        if img_result.is_some() {
            rst.borrow_mut().pimg(&img_result.unwrap(), x as i64, y as i64);
        }
        
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("pimg", fn_pimg);

    let rst = rasterizer.clone();
    let imga = assets_images.clone();
    let fn_pimgrect = lua.create_function(move |_, (name, x, y, image_x, image_y, image_width, image_height ): (String, f64, f64, f64, f64, f64, f64)| {
        //let imga_ref = imga.get();
        let img_result = imga.get(&name);
        if img_result.is_some() {
            rst.borrow_mut().pimgrect(&img_result.unwrap(), x as i64, y as i64, image_x as i64, image_y as i64, image_width as i64, image_height as i64);
        }
        
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("pimgrect", fn_pimgrect);

    // pimgmtx //
    let imga = assets_images.clone();
    let rst = rasterizer.clone();
    let fn_pimgmtx = lua.create_function(move |_, (name, x, y, r, sx, sy, ox, oy): (String, f64, f64, f64, f64, f64, f64, f64)| {
        //let imga_ref = imga.get();
        let img_result = imga.get(&name);
        if img_result.is_some() {
            rst.borrow_mut().pimgmtx(&img_result.unwrap(), x, y, r, sx, sy, ox, oy);
        }
        
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("pimgmtx", fn_pimgmtx);

    // pprint //
    let rst = rasterizer.clone();
    let fn_pprint = lua.create_function(move |_, (font, text, x, y): (Font, String, f64, f64)| {
        rst.borrow_mut().rasterizer.pprint(&font, text, x as i64, y as i64, 2, None);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("pprint", fn_pprint);
}