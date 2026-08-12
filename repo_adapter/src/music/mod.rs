// repo_adapter/src/music/mod.rs
// 🔌 插头 - 可乐音乐 - 模块
// 2026/8/10 20:00 Updated.

////////

use port::cola_music::ColaMusicPort;
use std::sync::Arc;

////////

pub mod add;
pub mod like;
pub mod view;

////////

/// # [BUILD] - 构建 MUSIC Port
/// * `desc`: 构建音乐 Port 聚合体
pub fn build_music_port() -> ColaMusicPort {
    ColaMusicPort {
        add: Arc::new(add::MusicAddPortAdapter),
        like: Arc::new(like::MusicLikePortAdapter),
        view: Arc::new(view::MusicViewPortAdapter),
    }
}

//////// END