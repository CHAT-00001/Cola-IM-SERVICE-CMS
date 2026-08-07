// cola_video/port/danmaku/mod.rs
// 视频 - port - 弹幕 - 模块
// 2026/8/5 15:15 Created.

////////

mod active;
mod add;
mod alive;
mod check;
mod del;
mod dislike;
mod get;
mod ids;
mod like;
mod list;
mod manage;
mod report;
mod stat;

pub use add::DanmakuAddPort;
use check::DanmakuCheckPort;
use del::DanmakuDelPort;
use get::DanmakuGetPort;
use like::DanmakuLikePort;
use list::DanmakuListPort;
use manage::DanmakuManagePort;
use report::DanmakuReportPort;
use stat::DanmakuStatPort;
use std::sync::Arc;

////////

/// # [DANMAKU PORT]
/// * `desc`: `弹幕 PortPort`
#[derive(Clone)]
pub struct DanmakuPort {
    pub add: Arc<dyn DanmakuAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn DanmakuCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn DanmakuDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn DanmakuGetPort + Send + Sync + 'static>, // 获取
    pub like: Arc<dyn DanmakuLikePort + Send + Sync + 'static>, // 点赞
    pub list: Arc<dyn DanmakuListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn DanmakuManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn DanmakuStatPort + Send + Sync + 'static>, // 状态
    pub report: Arc<dyn DanmakuReportPort + Send + Sync + 'static>, // 举报
    pub step: Arc<dyn DanmakuLikePort + Send + Sync + 'static>, // 踩踏
}

//////// END
