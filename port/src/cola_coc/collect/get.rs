// cola_video/port/collect/get.rs
// ▶ 可乐视频 - port - 收藏 - 获取
// 2026/8/5 00:04 Created.

////////

////////

/// # [DEL PORTS] - 获取
/// `desc`: `▶ 可乐视频 - 收藏获取端口`
#[async_trait::async_trait]
pub trait VideoCollectGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 用户的
    /// * `desc`: `根据用户ID` - `批量获取视频IDs`
    async fn get_video_ids_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
