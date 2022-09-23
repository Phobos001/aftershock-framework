use rgb::ComponentBytes;

use crate::rasterizer::Rasterizer;

pub fn get_error_bg() -> Rasterizer {
    let decode_result = lodepng::decode32(ERROR_BG);
        if decode_result.is_ok() {
            let img = decode_result.unwrap();
            let mut r = Rasterizer::new(img.width, img.height);
            r.color = img.buffer.as_bytes().to_vec();
            r
        } else {
            panic!("ERROR - ERROR: Error screen is broken; ERROR_BG cannot be decoded!");
        }
}

pub fn get_tiny_font() -> Rasterizer {
    let decode_result = lodepng::decode32(TINY_FONT);
        if decode_result.is_ok() {
            let img = decode_result.unwrap();
            let mut r = Rasterizer::new(img.width, img.height);
            r.color = img.buffer.as_bytes().to_vec();
            r
        } else {
            panic!("ERROR - ERROR: Error screen is broken; TINY_FONT cannot be decoded!");
        }
}

const ERROR_BG: &[u8] = include_bytes!("error_bg.png");
const TINY_FONT: &[u8] = include_bytes!("tiny_font10.png");
