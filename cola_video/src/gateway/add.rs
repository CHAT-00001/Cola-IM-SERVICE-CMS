// cola_video gateway/add.rs  -- 添加
// 2026/6/4 15:41 by wx: cestbon10080

////////

/// # [ACTION] - 动作
pub enum AddAction {
    publish = 1001, // 发布视频
    comment = 1002, // 发表评论
    danmaku = 1003, // 发表弹幕
}
