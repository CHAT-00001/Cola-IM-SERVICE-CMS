// repo_adapter/src/market/comment/mod.rs
// 🔌 适配器 - MARKET - 商品评论 - 模块
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
/// * `desc`: 购买订单评论端口构造器

pub fn build_comment_port() -> GoodsCommentPort {
    GoodsCommentPort {
        add: Arc::new(add::CommentAddPortAdapter),
        check: Arc::new(check::VideoCommentCheckAdapter),
        del: Arc::new(del::VideoCommentDelAdapter),
        dislike: Arc::new(dislike::VideoCommentDislikeAdapter),
        get: Arc::new(get::VideoCommentLikeAdapter),
        like: Arc::new(like::VideoCommentLikeAdapter),
        list: Arc::new(list::VideoCommentListAdapter),
        manage: Arc::new(manage::VideoCommentManageAdapter),
        stat: Arc::new(stat::VideoCommentStatAdapter),
    }
}

//////// END
