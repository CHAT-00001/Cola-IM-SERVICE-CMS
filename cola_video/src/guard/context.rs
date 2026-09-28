// cola_video/src/guard/context.rs -- VIDEO - GUARD - 守卫上下文
// 2026/9/29 Created.

////////

use super::relation::AuthorRelation;

////////

/// # [GUARD] - 视频守卫上下文
/// * `desc`: `由 CASE 组装，供评论、弹幕、下载、收藏、分享等策略复用`
#[derive(Debug, Clone, Copy)]
pub struct VideoGuardContext {
    pub requester_uid: i64,       // 请求者 UID
    pub author_uid: i64,          // 视频作者 UID
    pub video_id: i64,            // 视频 ID
    pub relation: AuthorRelation, // 请求者与作者关系
}

//////// END
