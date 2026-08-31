// repo_adapter/src/music/user/mod.rs -- 🔌 适配器 - MUSIC - 用户资料 - mod
// 2026/8/23 00:20 Created.

////////

use port::cola_music::user::MusicUserPort;
use std::sync::Arc;

////////

pub mod add;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [BUILD] - 构建音乐用户资料 Port
/// * `desc`: `音乐用户资料适配器`
pub fn build_music_user_port() -> MusicUserPort {
    MusicUserPort {
        add: Arc::new(add::MusicUserAddAdapter),
        check: Arc::new(check::MusicUserCheckAdapter),
        del: Arc::new(del::MusicUserDelAdapter),
        get: Arc::new(get::MusicUserGetAdapter),
        list: Arc::new(list::MusicUserListAdapter),
        manage: Arc::new(manage::MusicUserManageAdapter),
        stat: Arc::new(stat::MusicUserStatAdapter),
    }
}

//////// END
