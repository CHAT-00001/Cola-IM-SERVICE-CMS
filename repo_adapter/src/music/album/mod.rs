// repo_adapter/src/music/album/mod.rs
// 🔌 适配器 - 可乐音乐 - 专辑
// 2026/8/23 00:20 Created.

////////

use port::cola_music::album::MusicAlbumPort;
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

/// # [BUILD] - 音乐专辑构建专辑 Port
/// * `desc`: `专辑创建、更新与删除适配器`
pub fn build_music_album_port() -> MusicAlbumPort {
    MusicAlbumPort {
        add: Arc::new(add::MusicAlbumAddAdapter),          // 发布
        check: Arc::new(check::MusicAlbumCheckAdapter),    // 检查
        delete: Arc::new(del::MusicAlbumDeleteAdapter),    // 删除
        get: Arc::new(get::MusicAlbumGetAdapter),          // 获取
        list: Arc::new(list::MusicAlbumListAdapter),       // 列表
        manage: Arc::new(manage::MusicAlbumManageAdapter), // 管理
        stat: Arc::new(stat::MusicAlbumStatAdapter),       // 统计
    }
}

//////// END
