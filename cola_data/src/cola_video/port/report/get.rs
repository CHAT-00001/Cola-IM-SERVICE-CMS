// cola_video/prot/report/get.rs
// 视频 - port - 举报 - 获取
// 2026/8/5 15:51 Created.

////////

////////

/// # [GET PORTS] - 评论
/// * `desc`: `视频举报获取端口`
#[async_trait::async_trait]
pub trait VideoReportGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 我的
    async fn get_my_record_ids(
        &self,
        uid: i64,    // 操作者ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - 他的
    async fn get_he_record_ids(
        &self,
        user_id: i64, // 用户ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
