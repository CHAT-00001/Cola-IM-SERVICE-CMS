// port/src/cola_dynamic/port/like/add.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 点赞 - 发布
// 2026/8/5 00:02 Created.

////////

////////

/// # [ADD SERVICE] - 点赞
/// * `desc`: `视频点赞发布端口`
#[async_trait::async_trait]
pub trait LikeAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 点赞/取消
    async fn upsert_like(
        &self,
        uid: i64,        // 操作者 ID
        dynamic_id: i64, // 视频 ID
        is_liked: bool,  // 是否点赞
    ) -> anyhow::Result<(bool)>;
}

//////// END
