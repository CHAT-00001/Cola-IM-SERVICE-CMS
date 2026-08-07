// dynamic/port/hotlist/get.rs
// 动态 - port - 上热门 - 存活
// 2026/8/5 00:07 Created.

////////

/// # [ALIVE PORTS] - 存活
/// `desc`: `动态上热门存活端口`
#[async_trait::async_trait]
pub trait HotlistAlivePort: Send + Sync {
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