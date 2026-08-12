// port/src/cola_video/port/danmaku/mod.rs
// ⏩️ 端口 - VIDEO - 弹幕 - mod
// 2026/8/5 15:15 Created.

////////

use crate::cola_video::danmaku::add::VideoDanmakuAddPort;
use crate::cola_video::danmaku::check::VideoDanmakuCheckPort;
use crate::cola_video::danmaku::del::VideoDanmakuDelPort;
use crate::cola_video::danmaku::get::VideoDanmakuGetPort;
use crate::cola_video::danmaku::like::VideoDanmakuLikePort;
use crate::cola_video::danmaku::list::VideoDanmakuListPort;
use crate::cola_video::danmaku::manage::VideoDanmakuManagePort;
use crate::cola_video::danmaku::stat::VideoDanmakuStatPort;
use std::sync::Arc;
use crate::cola_video::danmaku::dislike::VideoDanmakuDislikePort;

////////
pub mod add; // 发布
pub mod alive; // 存活
pub mod check; // 检查
pub mod del; // 软删除
pub mod dislike; // 不喜欢
pub mod get; // 获取
pub mod like; // 点赞
pub mod list;  // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [VIDEO DANMAKU PORT]
/// * `desc`: `视频弹幕 Portst`
#[derive(Clone)]
pub struct VideoDanmakuPort {
    pub add: Arc<dyn VideoDanmakuAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn VideoDanmakuCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn VideoDanmakuDelPort + Send + Sync + 'static>, // 删除
    pub dislike: Arc<dyn VideoDanmakuDislikePort + Send + Sync + 'static>, // 不喜欢
    pub get: Arc<dyn VideoDanmakuGetPort + Send + Sync + 'static>, // 获取
    pub like: Arc<dyn VideoDanmakuLikePort + Send + Sync + 'static>, // 点赞
    pub list: Arc<dyn VideoDanmakuListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn VideoDanmakuManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VideoDanmakuStatPort + Send + Sync + 'static>, // 状态
}

//////// END
