// port/src/cola_dynamic/danmaku/dislike.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 弹幕 - 不喜欢
// 2026/8/9 03:19 Created.

////////


////////

/// # [DISLIKE PORTS] - 不喜欢
/// * `desc`: `⏹ 可乐动态 - 弹幕不喜欢端口`
#[async_trait::async_trait]
pub trait DynamicDanmakuDislikePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 不喜欢
    /// * `desc`: `🗣 用户` - 发布不喜欢弹幕
    async fn upsert_dislike(
        &self,
        uid: i64,        // UID
        danmaku_id: i64, // 弹幕 ID
        state: bool,     // 状态
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 检查状态
    /// * `desc`: `👀 AUTO` - 检查是否不喜欢弹幕
    async fn check_state(
        &self,
        uid: i64,            // UID
        danmaku_id: i64,     // 弹幕ID
    ) -> anyhow::Result<(bool)>;
}

//////// END