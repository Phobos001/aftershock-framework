use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;

use dashmap::DashMap;
use soloud::{Soloud, Wav, WavStream};

use crate::squaresrng::SquaresRNG;
use crate::VideoData;
use crate::rasterizer::Rasterizer;
use crate::controls::ControlData;

pub type SharedVideoData = Rc<RefCell<VideoData>>;
pub type SharedRasterizer = Rc<RefCell<Rasterizer>>;
pub type SharedControlData = Rc<RefCell<ControlData>>;
pub type SharedRNG = Rc<RefCell<SquaresRNG>>;

pub type SharedAudio = Arc<Soloud>;
pub type SharedAudioWav = Arc<DashMap<String, Wav>>;
pub type SharedAudioWavStream = Arc<DashMap<String, WavStream>>;

pub type SharedImages = Arc<DashMap<String, Rasterizer>>;