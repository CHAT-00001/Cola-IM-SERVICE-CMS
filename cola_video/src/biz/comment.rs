// cola_video/src/video/biz/comment.rs  -- 短视频 - 评论业务（占位）
// 2026/5/23 by wx: cestbon10080

////////

use anyhow::Result;

/// # LOGIC - 获取评论列表（占位）
pub async fn logic_get_comment_list(
    _video_id: i64,
    _page: i32,
    _size: i32,
) -> Result<String> {
    Ok("ok".to_string())
}

/// # LOGIC - 发送评论（占位）
pub async fn logic_add_comment(
    _uid: i64,
    _video_id: i64,
    _content: String,
) -> Result<()> {
    Ok(())
}
