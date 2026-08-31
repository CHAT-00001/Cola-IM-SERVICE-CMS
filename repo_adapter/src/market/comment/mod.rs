// repo_adapter/src/market/comment/mod.rs -- 适配器 - MARKET - 商品评论 - 模块
// 2026/8/8 Created.

////////

use port::market::comment::GoodsCommentPort;
use std::sync::Arc;

pub mod add;
pub mod alive;
pub mod check;
pub mod del;
pub mod dislike; // 不喜欢评论
pub mod get;
pub mod like; // 点赞评论
pub mod list;
pub mod manage;
pub mod stat;

////////

/// # [BUILD] - 构建 BUY Port
/// * `desc`: 购买商品订单评论端口构造器

pub fn build_goods_comment_port() -> GoodsCommentPort {
    GoodsCommentPort {
        add: Arc::new(add::GoodsCommentAddAdapter),
        check: Arc::new(check::GoodsCommentCheckAdapter),
        del: Arc::new(del::GoodsCommentDelAdapter),
        dislike: Arc::new(dislike::GoodsCommentDislikeAdapter),
        get: Arc::new(get::GoodsCommentLikeAdapter),
        like: Arc::new(like::GoodsCommentLikeAdapter),
        list: Arc::new(list::GoodsCommentListAdapter),
        manage: Arc::new(manage::GoodsCommentManageAdapter),
        stat: Arc::new(stat::GoodsCommentStatAdapter),
    }
}

//////// END
