use std::rc::Rc;
use std::cell::RefCell;

use dashmap::DashMap;
use soloud::{Soloud, Wav, WavStream};

//use rapier2d_f64::prelude::*;

use crate::partitioned_rasterizer::PartitionedRasterizer;
use crate::VideoData;
use crate::rasterizer::Rasterizer;
use crate::controls::ControlData;
//use crate::rapier2d_wrap::RapierWorld2D;

pub type SharedVideoData = Rc<RefCell<VideoData>>;
pub type SharedRasterizer = Rc<RefCell<PartitionedRasterizer>>;
pub type SharedControlData = Rc<RefCell<ControlData>>;
//pub type SharedPhysics2D = Rc<RefCell<RapierWorld2D>>;

pub type SharedAudio = Rc<Soloud>;
pub type SharedAudioHandle = Rc<DashMap<String, soloud::Handle>>;
pub type SharedAudioWav = Rc<DashMap<String, Wav>>;
pub type SharedAudioWavStream = Rc<DashMap<String, WavStream>>;

pub type SharedImages = Rc<DashMap<String, Rasterizer>>;

