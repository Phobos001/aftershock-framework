use mlua::Lua;
use crate::squaresrng::SquaresRNG;

use crate::{api_shareables::*, keys};

pub fn register_rng_api(rng: SharedRNG, lua: &Lua) {

    let random = rng.clone();
    let fn_randomize = lua.create_function( move |_, ()| {
        let r = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("DONT. FUCK. WITH TIME.").as_secs();
        *random.borrow_mut() = SquaresRNG { counter: r, key: keys::KEYS_TABLE[(r as usize % keys::KEYS_TABLE.len())] };
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("randomize", fn_randomize);

    let random = rng.clone();
    let fn_rand = lua.create_function( move |_, ()| {
        Ok(random.borrow_mut().randf64())
    }).unwrap();
    let _ = lua.globals().set("rand", fn_rand);

    let random = rng.clone();
    let fn_rand_range = lua.create_function( move |_, (min, max): (f64, f64)| {
        Ok(random.borrow_mut().rangef64(min, max))
    }).unwrap();
    let _ = lua.globals().set("rand_range", fn_rand_range);
}