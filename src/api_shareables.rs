use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;

use dashmap::DashMap;
use soloud::{Soloud, Wav, WavStream};

use rapier2d_f64::prelude::*;

use crate::partitioned_rasterizer::PartitionedRasterizer;
use crate::VideoData;
use crate::rasterizer::Rasterizer;
use crate::controls::ControlData;
use crate::api_physics::RapierWorld2D;

pub type SharedVideoData = Rc<RefCell<VideoData>>;
pub type SharedRasterizer = Rc<RefCell<PartitionedRasterizer>>;
pub type SharedControlData = Rc<RefCell<ControlData>>;
pub type SharedPhysics2D = Rc<RefCell<RapierWorld2D>>;

pub type SharedAudio = Arc<Soloud>;
pub type SharedAudioWav = Arc<DashMap<String, Wav>>;
pub type SharedAudioWavStream = Arc<DashMap<String, WavStream>>;

pub type SharedImages = Arc<DashMap<String, Rasterizer>>;

