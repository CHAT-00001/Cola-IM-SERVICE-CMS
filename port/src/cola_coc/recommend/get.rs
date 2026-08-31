// cola_video/port/hotlist/get.rs
// 视频 - port - 推荐 - 获取
// 2026/8/5 00:01 Created.

////////

/// # [DEL PORTS] - 获取
/// `desc`: `视频收藏获取端口`
#[async_trait::async_trait]
pub trait VideoRecommendGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `用户批量获取收藏的视频IDs`
    async fn get_my_collect_ids(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - TA
    /// * `desc`: `用户批量获取收藏的视频IDs`
    async fn get_he_collect_ids(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
