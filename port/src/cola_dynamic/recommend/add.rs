// cola_video/port/recommend/add.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 推荐 - 列表
// 2026/8/5 00:01 Created.

////////


////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `⏹ 可乐动态 - 推荐发布端口`
#[async_trait::async_trait]
pub trait DynamicRecommendAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 发送
    /// * `desc`: `🗣 USER` - `开始推荐/取消推荐 动态`
    async fn upsert_recommend(
        &self,
        uid: i64,           // UID
        dynamic_id: i64,    // 动态 ID
        is_recommend: bool, // 是否点赞
    ) -> anyhow::Result<(u16)>;
}

//////// END
