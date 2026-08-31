// port/src/cola_live/stream/mod.rs
// 端口 - LIVE - 直播场次 - 三层端口聚合
// 2026/8/21 09:32 Created.

////////

use crate::cola_live::stream::add::LiveStreamAddPort;
use crate::cola_live::stream::check::LiveStreamCheckPort;
use crate::cola_live::stream::get::LiveStreamGetPort;
use crate::cola_live::stream::list::LiveStreamListPort;
use crate::cola_live::stream::manage::LiveStreamManagePort;
use std::sync::Arc;

////////

pub mod add;
pub mod check;
pub mod get;
pub mod list;
pub mod manage;

////////

/// # 1. [PORTS] - 直播场次端口
#[derive(Clone)]
pub struct LiveStreamPort {
    pub add: Arc<dyn LiveStreamAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn LiveStreamCheckPort + Send + Sync + 'static>,
    pub get: Arc<dyn LiveStreamGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn LiveStreamListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn LiveStreamManagePort + Send + Sync + 'static>,
}

//////// END
