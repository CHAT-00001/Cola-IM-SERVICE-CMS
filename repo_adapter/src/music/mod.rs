// repo_adapter/src/music/mod.rs
// 🔌 插头 - 可乐音乐 - 模块
// 2026/8/10 20:00 Updated.

////////

use port::cola_music::ColaMusicPort;
use std::sync::Arc;

// 注：音乐 Port 复用 GIS 的 Adapter
use crate::gis;

////////

pub mod add;
pub mod like;
pub mod view;

////////

/// # [BUILD] - 构建 MUSIC Port
/// * `desc`: 构建音乐 Port 聚合体，复用 GIS 相关功能
pub fn build_music_port() -> ColaMusicPort {
    ColaMusicPort {
        add: Arc::new(gis::add::AddPortAdapter),
        like: Arc::new(gis::like::LikePortAdapter),
        view: Arc::new(gis::view::ViewPortAdapter),
    }
}

//////// END