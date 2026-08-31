// repo_adapter/src/coc/mod.rs -- 🔌 插头 - COC - mod
// 2026/8/10 20:00 Created.

////////

use port::cola_coc::ColaCocPort;
use port::cola_video::ColaVideoPort;

////////

pub mod buy; // 购买
pub mod collect; // 收藏
pub mod comment; // 评论
pub mod danmaku; // 弹幕
pub mod dislike; // 不喜欢
pub mod hotlist; // 上热门
pub mod like; // 点赞
pub mod recommend; // 推荐
pub mod share; // 分享
pub mod video; // 视频(主内容)
pub mod view; // 浏览

////////

/// # [BUILD] - 构建 COC Port
/// * `desc`: 构建视运营中心 Port 聚合体，包含所有视频相关功能
pub fn build_cola_coc_port() -> ColaCocPort {
    ColaCocPort {
        buy: buy::build_video_buy_port(),
        collect: collect::build_video_collect_port(),
        comment: comment::build_video_comment_port(),
        danmaku: danmaku::build_video_danmaku_port(),
        dislike: dislike::build_video_dislike_port(),
        hotlist: hotlist::build_video_hotlist_port(),
        like: like::build_video_like_port(),
        recommend: recommend::build_video_recommend_port(),
        share: share::build_video_share_port(),
        video: video::build_video_video_port(),
        view: view::build_video_view_port(),
    }
}

//////// END
