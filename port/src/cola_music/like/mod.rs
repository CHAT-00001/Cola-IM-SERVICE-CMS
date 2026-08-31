// port/src/cola_music/like/mod.rs
// ⏩️ 端口 - 可乐音乐 - 点赞 - mod
// 2026/8/23 00:10 Created.

////////

use crate::cola_music::like::add::MusicLikeAddPort;
use crate::cola_music::like::del::MusicLikeDelPort;
use crate::cola_music::like::get::MusicLikeGetPort;
use crate::cola_music::like::list::MusicLikeListPort;
use crate::cola_music::like::manage::MusicLikeManagePort;
use crate::cola_music::like::stat::MusicLikeStatPort;
use std::sync::Arc;
////////

pub mod add;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [MUSIC FAVORITES PORTS] - 音乐最喜欢端口
/// * `desc`: `音乐最喜欢关系的完整端口聚合`
#[derive(Clone)]
pub struct MusicLikePort {
    pub add: Arc<dyn MusicLikeAddPort + Send + Sync + 'static>, // 新增/恢复
    pub get: Arc<dyn MusicLikeGetPort + Send + Sync + 'static>, // 获取音乐 IDs
    pub list: Arc<dyn MusicLikeListPort + Send + Sync + 'static>, // 审计记录
    pub manage: Arc<dyn MusicLikeManagePort + Send + Sync + 'static>, // 管理
    pub del: Arc<dyn MusicLikeDelPort + Send + Sync + 'static>, // 硬删除
    pub stat: Arc<dyn MusicLikeStatPort + Send + Sync + 'static>, // 统计
}

//////// END
