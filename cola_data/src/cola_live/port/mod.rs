// cola_live/port/mod.rs
// 直播 - port - 模块
// 2026/6/10 07:44

////////

use crate::cola_gis::port::add::AddPort;
use crate::cola_gis::port::like::LikeRepo;
use crate::cola_gis::port::view::ViewPort;
use std::sync::Arc;

////////

/// # [COLA LIVE PORTS]
/// * `desc`: `可乐直播端口`
#[derive(Clone)]
pub struct ColaLivePort {
    pub add: Arc<dyn AddPort + Send + Sync + 'static>,
    // pub feed: Arc<dyn FeedPort>,
    // pub collect: Arc<dyn CollectPort>,
    // pub state: Arc<dyn StatePort>,
    pub like: Arc<dyn LikeRepo + Send + Sync + 'static>,
    pub view: Arc<dyn ViewPort + Send + Sync + 'static>,
}

//////// END