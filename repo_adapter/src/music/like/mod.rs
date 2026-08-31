// repo_adapter/src/music/like/mod.rs -- 适配器 - MUSIC - 点赞 - mod
// 2026/8/23 00:20 Created.

////////

use port::cola_music::like::MusicLikePort;
use std::sync::Arc;

////////

pub mod add;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [BUILD] - 构建点赞 Port
/// * `desc`: `分别装配 add、get、list、manage、del、stat`
pub fn build_music_like_port() -> MusicLikePort {
    MusicLikePort {
        add: Arc::new(add::MusicLikeAddAdapter),
        get: Arc::new(get::MusicLikeGetAdapter),
        list: Arc::new(list::MusicLikeListAdapter),
        manage: Arc::new(manage::MusicLikeManageAdapter),
        del: Arc::new(del::MusicLikeDelAdapter),
        stat: Arc::new(stat::MusicLikeStatAdapter),
    }
}

//////// END