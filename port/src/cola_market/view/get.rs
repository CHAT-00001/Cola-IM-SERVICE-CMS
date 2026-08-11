// port/src/cola_video/view/get.rs
// ⏩️ 端口 - ▶ 可乐视频 - 浏览 获取
// 2026/8/4 22:10 Created.

////////

////////

/// # [GET  PORTS]
/// * `desc`: `VIDEO - 视频浏览记录获取端口`
#[async_trait::async_trait]
pub trait VideoViewGetPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 获取用户浏览的视频IDs
    async fn get_video_ids_by_user_id(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
