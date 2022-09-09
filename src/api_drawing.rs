use crate::color::*;
use crate::rasterizer::*;
use crate::font::*;

use mlua::prelude::*;

use crate::api_shareables::*;

pub fn register_draw_api(assets_images: SharedImages, rasterizer: SharedRasterizer, lua: &Lua) {

    let rst = rasterizer.clone();
    let fn_get_collected_pixels = lua.create_function(move |luamove, ()| {
        let list = luamove.create_table().unwrap();
        let xlist = luamove.create_table().unwrap();
        let ylist = luamove.create_table().unwrap();

        let rst_ref = rst.borrow_mut();
        let pixels = &rst_ref.collected_pixels;
        for i in 0..pixels.len() {
            let _ = xlist.set(i, pixels[i].0);
            let _ = ylist.set(i, pixels[i].1);
        }
        let _ = list.set("x", xlist);
        let _ = list.set("y", ylist);
        Ok(list)
    }).unwrap();
    //let _ = lua.globals().set("get_collected_pixels", fn_get_collected_pixels);
    // Slower than just calling pset, disabled

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
    let _ = lua.globals().set("set_draw_mode_subtraction", fn_set_draw_mode_multiply);

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
        rst.borrow_mut().ptriangle(filled, x0, y0, x1, y1, x2, y2, color);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("ptriangle", fn_ptriangle);

    // pbeizer //
    let rst = rasterizer.clone();
    let fn_pbeizer = lua.create_function(move |_, (thickness, x0, y0, x1, y1, mx, my, color): (i64, i64, i64, i64, i64, i64, i64, Color)| {
        rst.borrow_mut().pbeizer(thickness, x0, y0, x1, y1, mx, my, color);
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

    // pimgrect //
    /*let assets = assets_image.clone();
    let rst = rasterizer.clone();
    engine.register_fn("pimgrect", move |image_name: ImmutableString, x: i64, y: i64, rx: i64, ry: i64, rw: i64, rh: i64| {
        
        let res = assets.borrow();
        if res.contains_key(&image_name) {
            let image = &*res.get(&image_name).unwrap();
            rst.borrow().pimgrect(image, x, y, rx, ry, rw, rh);
        }
    } ); */

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
        rst.borrow_mut().pprint(&font, text, x as i64, y as i64, 2, None);
        Ok(())
    } ).unwrap();
    let _ = lua.globals().set("pprint", fn_pprint);
}