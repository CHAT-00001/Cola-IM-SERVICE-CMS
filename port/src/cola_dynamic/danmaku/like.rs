// port/src/cola_dynamic/danmaku/like.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 弹幕 - 不喜欢
// 2026/8/9 03:25 Created.

////////

////////

/// # [LIKE PORTS] - 点赞
/// * `desc`: `⏹ 可乐动态 - 弹幕点赞端口`
#[async_trait::async_trait]
pub trait DynamicDanmakuLikePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 点赞
    /// * `desc`: `🗣 用户` - 发布点赞弹幕
    async fn upsert_like(
        &self,
        uid: i64,        // UID
        danmaku_id: i64, // 弹幕 ID
        state: bool,     // 状态
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 检查状态
    /// * `desc`: `👀 AUTO` - 检查是否点赞弹幕
    async fn check_state(
        &self,
        uid: i64,        // UID
        danmaku_id: i64, // 弹幕ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
