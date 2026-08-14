// port/src/market/comment/mod.rs
// ⏩️ 端口 - MARKET - 商品评论 - mod
// 2026/8/5 15:11 Created.

////////

use crate::market::comment::add::GoodsCommentAddPort;
use crate::market::comment::check::GoodsCommentCheckPort;
use crate::market::comment::del::GoodsCommentDelPort;
use crate::market::comment::dislike::GoodsCommentDisikePort;
use crate::market::comment::get::GoodsCommentGetPort;
use crate::market::comment::like::GoodsCommentLikePort;
use crate::market::comment::list::GoodsCommentListPort;
use crate::market::comment::manage::GoodsCommentManagePort;
use crate::market::comment::stat::GoodsCommentStatPort;
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

/// # [GOODS COMMENT PORT]
/// * `desc`: `MARKET - 商品评论 Ports`
#[derive(Clone)]
pub struct GoodsCommentPort {
    pub add: Arc<dyn GoodsCommentAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn GoodsCommentCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn GoodsCommentDelPort + Send + Sync + 'static>, // 删除
    pub dislike: Arc<dyn GoodsCommentDisikePort + Send + Sync + 'static>, // 不喜欢
    pub get: Arc<dyn GoodsCommentGetPort + Send + Sync + 'static>, // 获取
    pub like: Arc<dyn GoodsCommentLikePort + Send + Sync + 'static>, // 点赞
    pub list: Arc<dyn GoodsCommentListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn GoodsCommentManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn GoodsCommentStatPort + Send + Sync + 'static>, // 统计
}

//////// END
