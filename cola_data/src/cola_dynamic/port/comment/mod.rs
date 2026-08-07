// cola_dynamic/port/comment/mod.rs
// 动态 - port - 评论 - 模块
// 2026/8/5 15:11 Created.

////////
use crate::cola_dynamic::port::comment::like::LikePort;
use crate::cola_dynamic::port::comment::list::ListPort;
use crate::cola_dynamic::port::comment::manage::ManagePort;
use crate::cola_dynamic::port::comment::report::ReportPort;
use crate::cola_dynamic::port::comment::stat::StatPort;
use crate::cola_dynamic::port::danmaku::DanmakuAddPort;
use check::CheckPort;
use del::DelPort;
use get::GetPort;
use std::sync::Arc;

////////
mod active; // 活跃
mod add; // 发布
mod check; // 检查
mod del; // 删除
mod get; // 获取
mod like; // 点赞
mod list; // 列表
mod manage; // 管理
mod report; // 举报
mod stat; // 统计

////////

/// # [COMMENT PORT]
/// * `desc`: `动态评论 Ports`
#[derive(Clone)]
pub struct DynamicCommentPort {
    pub add: Arc<dyn DanmakuAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn CheckPort + Send + Sync + 'static>,    // 检查
    pub del: Arc<dyn DelPort + Send + Sync + 'static>,        // 删除
    pub get: Arc<dyn GetPort + Send + Sync + 'static>,        // 获取
    pub like: Arc<dyn LikePort + Send + Sync + 'static>,      // 点赞
    pub list: Arc<dyn ListPort + Send + Sync + 'static>,      // 列表
    pub manage: Arc<dyn ManagePort + Send + Sync + 'static>,  // 管理
    pub report: Arc<dyn ReportPort + Send + Sync + 'static>,  // 举报
    pub stat: Arc<dyn StatPort + Send + Sync + 'static>,      // 统计
}

//////// END
