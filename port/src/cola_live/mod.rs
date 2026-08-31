// port/src/cola_live/mod.rs -- 端口 - 可乐直播 - mod
// 2026/6/10 07:44 Created.

////////

use crate::cola_gis::add::AddPort;
use crate::cola_gis::like::LikeRepo;
use crate::cola_gis::view::ViewPort;
use category::LiveCategoryPort;
use std::sync::Arc;
use user::LiveUserPort;

////////

pub mod category; // 分类
pub mod stream; // 流
pub mod user; // 用户资料

////////

/// # [COLA LIVE PORTS]
/// * `desc`: `可乐直播端口`
#[derive(Clone)]
pub struct ColaLivePort {
    pub stream: stream::LiveStreamPort,
    pub category: Arc<dyn LiveCategoryPort + Send + Sync + 'static>,
    pub add: Arc<dyn AddPort + Send + Sync + 'static>,
    // pub feed: Arc<dyn FeedPort>,
    // pub collect: Arc<dyn CollectPort>,
    // pub state: Arc<dyn StatePort>,
    pub like: Arc<dyn LikeRepo + Send + Sync + 'static>,
    pub view: Arc<dyn ViewPort + Send + Sync + 'static>,
    pub user: Arc<dyn LiveUserPort + Send + Sync + 'static>, // 直播域用户
}

//////// END
