// repo_adapter/src/live/mod.rs
// 🔌 插头 - 可乐直播 - 模块
// 2026/8/10 20:00 Updated.

////////

use port::cola_live::ColaLivePort;
use std::sync::Arc;

// 注：直播 Port 复用 GIS 的 Adapter
use crate::gis;

////////

/// # [BUILD] - 构建 LIVE Port
/// * `desc`: 构建直播 Port 聚合体，复用 GIS 相关功能
pub fn build_live_port() -> ColaLivePort {
    ColaLivePort {
        add: Arc::new(gis::add::AddPortAdapter),
        like: Arc::new(gis::like::LikePortAdapter),
        view: Arc::new(gis::view::ViewPortAdapter),
    }
}

//////// END

