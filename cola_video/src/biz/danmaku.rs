// cola_video/src/live/biz/danmaku  -- 短视频 - 弹幕业务
// 2026/4/24 19:34 by wx: cestbon10080

////////

use crate::video::port::view::ViewPort;
use anyhow::Result;

/// # LOGIC - 获取弹幕列表（占位）
pub async fn logic_get_danmaku_segment(
    _user_id: i64,
    _video_id: i64,
    _start_time: i32,
    _end_time: i32,
    _limit: i32,
) -> Result<String> {
    Ok("ok".to_string())
}

/// # LOGIC - 发送弹幕（占位）
pub async fn logic_add_danmaku(
    _user_id: i64,
    _video_id: i64,
    _content: String,
    _play_time: i32,
) -> Result<()> {
    Ok(())
}
