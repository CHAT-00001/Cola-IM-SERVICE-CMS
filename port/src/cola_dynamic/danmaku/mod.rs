// port/src/cola_dynamic/port/danmaku/music.rs
// ⏩️ 端口 - 可乐动态 - 弹幕 - 模块
// 2026/8/9 04:14 Created.

////////

use crate::cola_dynamic::danmaku::add::DynamicDanmakuAddPort;
use crate::cola_dynamic::danmaku::check::DynamicDanmakuCheckPort;
use crate::cola_dynamic::danmaku::del::DynamicDanmakuDelPort;
use crate::cola_dynamic::danmaku::dislike::DynamicDanmakuDislikePort;
use crate::cola_dynamic::danmaku::get::DynamicDanmakuGetPort;
use crate::cola_dynamic::danmaku::like::DynamicDanmakuLikePort;
use crate::cola_dynamic::danmaku::list::DynamicDanmakuListPort;
use crate::cola_dynamic::danmaku::manage::DynamicDanmakuManagePort;
use crate::cola_dynamic::danmaku::stat::DynamicDanmakuStatPort;
use std::sync::Arc;

////////
pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod dislike; // 不喜欢
pub mod get; // 获取
pub mod ids; // IDs
pub mod like; // 点赞
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [DYNAMIC DANMAKU PORTS]
/// * `desc`: `⏹ 可乐动态 - 弹幕 PortPort`
#[derive(Clone)]
pub struct DynamicDanmakuPort {
    pub add: Arc<dyn DynamicDanmakuAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn DynamicDanmakuCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn DynamicDanmakuDelPort + Send + Sync + 'static>, // 删除
    pub dislike: Arc<dyn DynamicDanmakuDislikePort + Send + Sync + 'static>, // 不喜欢
    pub get: Arc<dyn DynamicDanmakuGetPort + Send + Sync + 'static>, // 获取
    pub like: Arc<dyn DynamicDanmakuLikePort + Send + Sync + 'static>, // 点赞
    pub list: Arc<dyn DynamicDanmakuListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn DynamicDanmakuManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn DynamicDanmakuStatPort + Send + Sync + 'static>, // 状态
    pub step: Arc<dyn DynamicDanmakuLikePort + Send + Sync + 'static>, // 踩踏
}

//////// END
