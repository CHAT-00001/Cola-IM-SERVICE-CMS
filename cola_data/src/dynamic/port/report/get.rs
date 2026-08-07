// dynamic/port/report/get.rs
// 动态 - port - 举报 - 获取
// 2026/8/5 15:51 Created.

////////

/// # [GET PORTS] - 获取
/// `desc`: `动态举报获取端口`
#[async_trait::async_trait]
pub trait ReportGetPort: Send + Sync {
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
