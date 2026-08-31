// repo_adapter/src/music/content/mod.rs
// 🔌 适配器 - 可乐音乐 - 主内容
// 2026/8/23 00:20 Created.

////////

use port::cola_music::music::MusicContentPort;
use std::sync::Arc;

////////

mod add;
mod check;
mod del;
mod get;
mod list;
mod manage;

////////

/// # [BUILD] - 构建音乐主内容 Port
/// * `desc`: `分别装配音乐主内容的 add、del、get、list、manage`
pub fn build_music_content_port() -> MusicContentPort {
    MusicContentPort {
        add: Arc::new(add::MusicAddAdapter),                 // 发布
        check: Arc::new(check::MusicContentCheckAdapter),    // 检查
        del: Arc::new(del::MusicContentDelAdapter),          // 删除
        get: Arc::new(get::MusicContentGetAdapter),          // 获取
        list: Arc::new(list::MusicContentListAdapter),       // 列表
        manage: Arc::new(manage::MusicContentManageAdapter), // 管理
    }
}

//////// END
