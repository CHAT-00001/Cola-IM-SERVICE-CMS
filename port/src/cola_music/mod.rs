// port/src/cola_music/mod.rs
// ⏩️ 端口 - 可乐音乐 - 模块聚合
// 2026/8/22 23:45 Created.

////////

use crate::cola_music::album::MusicAlbumPort;
use crate::cola_music::collect::MusicCollectPort;
use crate::cola_music::like::MusicLikePort;
use crate::cola_music::music::MusicContentPort;
use crate::cola_music::user::MusicUserPort;
use crate::cola_music::view::MusicViewPort;

////////

pub mod album; // 专辑
pub mod collect; // 收藏
pub mod feed; // 流
pub mod home; // 主页
pub mod like; // 点赞
pub mod music; // 内容
pub mod user; // 用户资料
pub mod view; // 浏览

////////

/// # [COLA MUSIC PORTS] - 音乐模块
/// * `desc`: `主内容与音乐关系模块的端口聚合`
#[derive(Clone)]
pub struct ColaMusicPort {
    pub music: MusicContentPort,   // 主内容
    pub album: MusicAlbumPort,     // 专辑
    pub collect: MusicCollectPort, // 收藏
    pub like: MusicLikePort,       // 点赞(预设)
    pub user: MusicUserPort,       // 用户
    pub view: MusicViewPort,       // 浏览(预设)
}

//////// END
