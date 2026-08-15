// port/src/cola_video/file/mod.rs
// ⏩️ 端口 - VIDEO - 评论 - 模块
// 2026/8/5 15:11 Created.

////////

use crate::cola_video::comment::add::VideoCommentAddPort;
use crate::cola_video::comment::check::VideoCommentCheckPort;
use crate::cola_video::comment::del::VideoCommentDelPort;
use crate::cola_video::comment::dislike::VideoCommentDisikePort;
use crate::cola_video::comment::get::VideoCommentGetPort;
use crate::cola_video::comment::like::VideoCommentLikePort;
use crate::cola_video::comment::list::VideoCommentListPort;
use crate::cola_video::comment::manage::VideoCommentManagePort;
use crate::cola_video::comment::stat::VideoCommentStatPort;
use std::sync::Arc;

////////
pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod dislike; // 不喜欢
pub mod get; // 获取
pub mod like; // 点赞
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [VIDEO COMMENT PORT]
/// * `desc`: `VIDEO - 评论 Ports`
#[derive(Clone)]
pub struct VideoCommentPort {
    pub add: Arc<dyn VideoCommentAddPort + Send + Sync + 'static>,        // 发布
    pub check: Arc<dyn VideoCommentCheckPort + Send + Sync + 'static>,    // 检查
    pub del: Arc<dyn VideoCommentDelPort + Send + Sync + 'static>,        // 删除
    pub dislike: Arc<dyn VideoCommentDisikePort + Send + Sync + 'static>, // 不喜欢
    pub get: Arc<dyn VideoCommentGetPort + Send + Sync + 'static>,        // 获取
    pub like: Arc<dyn VideoCommentLikePort + Send + Sync + 'static>,      // 点赞
    pub list: Arc<dyn VideoCommentListPort + Send + Sync + 'static>,      // 列表
    pub manage: Arc<dyn VideoCommentManagePort + Send + Sync + 'static>,  // 管理
    pub stat: Arc<dyn VideoCommentStatPort + Send + Sync + 'static>,      // 统计
}

//////// END