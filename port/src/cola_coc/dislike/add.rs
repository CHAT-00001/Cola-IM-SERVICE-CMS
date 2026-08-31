// port/src/videodislike/add.rs
// ⏩️ 端口 - ▶ 视频 - 不喜欢 - 发布
// 2026/8/5 15:59 Created.

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `不喜欢发布服务端口`
#[async_trait::async_trait]
pub trait VideoDislikeAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 发布不喜欢
    /// * `desc`: `根据用户ID + 视频ID` - `更新/插入不喜欢记录`
    async fn add_dislike(
        &self,
        uid: i64,      // UID
        user_id: i64,  // 用户 ID
        video_id: i64, // 视频 ID
        dislike: bool, // 是否不喜欢
    ) -> anyhow::Result<(bool)>;
}

//////// END
