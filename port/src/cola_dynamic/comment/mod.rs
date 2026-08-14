// port/src/cola_dynamic/identity/mod.rs
// ⏩️ 端口 - 可怜动态 - 评论 - 模块
// 2026/8/5 15:11 Created.

////////

use crate::cola_dynamic::comment::add::DynamicCommentAddPort;
use crate::cola_dynamic::comment::check::DynamicCommentCheckPort;
use crate::cola_dynamic::comment::get::DynamicCommentGetPort;
use crate::cola_dynamic::comment::like::DynamicCommentLikePort;
use crate::cola_dynamic::comment::list::DynamicCommentListPort;
use crate::cola_dynamic::comment::manage::DynamicCommentManagePort;
use crate::cola_dynamic::comment::report::DynamicCommentReportPort;
use crate::cola_dynamic::comment::stat::CommentStatPort;
use del::DelPort;
use std::sync::Arc;
use crate::cola_dynamic::comment::dislike::DynamicCommentDisikePort;

////////
pub mod active; // 活跃
pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod dislike; // 不喜欢
pub mod get; // 获取
pub mod like; // 点赞
pub mod list; // 列表
pub mod manage; // 管理
pub mod report; // 举报
pub mod stat; // 统计


////////

/// # [DYNAMIC COMMENT PORT]
/// * `desc`: `⏹ 可乐动态 - 评论 Ports`
#[derive(Clone)]
pub struct DynamicCommentPort {
    pub add: Arc<dyn DynamicCommentAddPort + Send + Sync + 'static>,         // 发布
    pub check: Arc<dyn DynamicCommentCheckPort + Send + Sync + 'static>,     // 检查
    pub del: Arc<dyn DelPort + Send + Sync + 'static>,                       // 删除
    pub dislike: Arc<dyn DynamicCommentDisikePort + Send + Sync + 'static>,  // 不喜欢
    pub get: Arc<dyn DynamicCommentGetPort + Send + Sync + 'static>,         // 获取
    pub like: Arc<dyn DynamicCommentLikePort + Send + Sync + 'static>,       // 点赞
    pub list: Arc<dyn DynamicCommentListPort + Send + Sync + 'static>,       // 列表
    pub manage: Arc<dyn DynamicCommentManagePort + Send + Sync + 'static>,   // 管理
    pub report: Arc<dyn DynamicCommentReportPort + Send + Sync + 'static>,   // 举报
    pub stat: Arc<dyn CommentStatPort + Send + Sync + 'static>,              // 统计
}

//////// END
