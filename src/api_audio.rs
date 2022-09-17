use mlua::prelude::*;
use soloud::prelude::*;

use crate::api_shareables::*;

pub fn register_audio_api(audio: SharedAudio, assets_sfx: SharedAudioWav, assets_mus: SharedAudioWavStream, lua: &Lua) {

    // SFX //
    let sfxa = assets_sfx.clone();
    let fn_load_sound = lua.create_function(move |_, (path_to, name): (String, String)| {
        // Overwrite anything already in the key
        let mut wav = soloud::audio::Wav::default();

        let wav_result = wav.load(&path_to);
        if wav_result.is_err() {
            println!("ERROR - AUDIO: Failed to load Wav at path '{}'! Soloud: {}", path_to, wav_result.err().unwrap());
        }
        sfxa.insert(name, wav);
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("load_sound", fn_load_sound);

    let sfxa = assets_sfx.clone();
    let fn_unload_sound = lua.create_function(move |_, name: String| {
        
        sfxa.remove(&name);
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("unload_sound", fn_unload_sound);

    let soloud = audio.clone();
    let sfxa = assets_sfx.clone();
    let fn_sfx = lua.create_function(move |_, name: String| {
        // Play sound, don't save handle
        let find_result = sfxa.get(&name);
        if find_result.is_some() {
            soloud.play(&*find_result.unwrap());
        }
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("play_sound", fn_sfx);

    // MUSIC //
    let musa = assets_mus.clone();
    let fn_load_mus = lua.create_function(move |_, (path_to, name): (String, String)| {
        // Overwrite anything already in the key
        let mut wav = soloud::audio::WavStream::default();

        let wav_result = wav.load(&path_to);
        if wav_result.is_err() {
            println!("ERROR - AUDIO: Failed to load Wav at path '{}'! Soloud: {}", path_to, wav_result.err().unwrap());
        }
        musa.insert(name, wav);
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("load_music", fn_load_mus);

    let musa = assets_mus.clone();
    let fn_unload_sound = lua.create_function(move |_, name: String| {

        musa.remove(&name);
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("unload_music", fn_unload_sound);

    let soloud = audio.clone();
    let musa = assets_mus.clone();
    let fn_mus = lua.create_function(move |_, name: String| {
        // Play sound, don't save handle
        let find_result = musa.get(&name);
        if find_result.is_some() {
            soloud.play(&*find_result.unwrap());
        }
        Ok(())
    }).unwrap();
    let _ = lua.globals().set("play_music", fn_mus);
}