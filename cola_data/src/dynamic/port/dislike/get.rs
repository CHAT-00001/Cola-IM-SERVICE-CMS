// dynamic/port/dislike/get.rs
// 动态 - port - 不喜欢 - 获取
// 2026/8/5 02:02 Created.

////////

////////

/// # [GET PORTS] - 获取
/// `desc`: `动态不喜欢获取端口`
#[async_trait::async_trait]
pub trait DislikeGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `用户批量获取不喜欢的视频IDs`
    async fn get_my_dislike_ids(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - TA
    /// * `desc`: `用户批量获取不喜欢的视频IDs`
    async fn get_he_dislike_ids(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END