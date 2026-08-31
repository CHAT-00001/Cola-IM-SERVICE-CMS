// repo_adapter/src/music/collect/mod.rs
// 🔌 适配器 - 可乐音乐 - 收藏
// 2026/8/23 00:20 Created.

////////

use port::cola_music::collect::MusicCollectPort;
use std::sync::Arc;

////////

pub mod add;
mod check;
mod del;
mod get;
mod list;
mod manage;
mod stat;

////////

/// # [BUILD] - 构建收藏 Port
/// * `desc`: `音乐收藏及指定专辑关系适配器`
pub fn build_music_collect_port() -> MusicCollectPort {
    MusicCollectPort {
        add: Arc::new(add::MusicCollectAddAdapter),
        check: Arc::new(check::MusicCollectCheckAdapter),
        delete: Arc::new(del::MusicCollectDeleteAdapter),
        get: Arc::new(get::MusicCollectGetAdapter),
        list: Arc::new(list::MusicCollectListAdapter),
        manage: Arc::new(manage::MusicCollectManageAdapter),
        stat: Arc::new(stat::MusicCollectStatAdapter),
    }
}

//////// END
