use mlua::prelude::*;

use std::time::{UNIX_EPOCH, SystemTime};

pub fn register_profiling_api(lua: &Lua) {
    let fn_timestamp = lua.create_function(move |_, ()| {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Ok((now.as_secs() as f64) + (now.subsec_nanos() as f64 * 0.000000001))
    }).unwrap();
    let _ = lua.globals().set("timestamp", fn_timestamp);
}