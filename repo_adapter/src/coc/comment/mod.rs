// repo_adapter/src/video/comment/mod.rs
// 🔌 适配器 - VIDEO - 评论 - music
// 2026/8/8 Created.

////////

use port::cola_video::comment::VideoCommentPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod alive; // 存活
pub mod check; // 检查
pub mod del; // 删除
pub mod dislike; // 不喜欢评论
pub mod get; // 获取
pub mod like; // 点赞评论
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 视频评论端口构造器
pub fn build_video_comment_port() -> VideoCommentPort {
    VideoCommentPort {
        add: Arc::new(add::CommentAddPortAdapter),
        check: Arc::new(check::VideoCommentCheckAdapter),
        del: Arc::new(del::VideoCommentDelAdapter),
        dislike: Arc::new(dislike::VideoCommentDislikeAdapter),
        get: Arc::new(get::VideoCommentGetAdapter),
        like: Arc::new(like::VideoCommentLikeAdapter),
        list: Arc::new(list::VideoCommentListAdapter),
        manage: Arc::new(manage::VideoCommentManageAdapter),
        stat: Arc::new(stat::VideoCommentStatAdapter),
    }
}

//////// END
