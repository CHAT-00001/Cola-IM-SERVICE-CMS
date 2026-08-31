// port/src/cola_video/buy/del.rs
// ⏩️ 端口 - ▶ 可乐视频 - 购买 - 删除
// 2026/8/5 01:52 Created.

////////

////////

/// # [DEL SERVICE] - 删除
/// `desc`: `视频购买删除端口`
#[async_trait::async_trait]
pub trait GoodsBuyDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    /// * `desc`: `用户单个软删除分享记录`
    async fn single_soft_del_record(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频ID
        id: i64,       // 目标ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 保存
    /// * `desc`: `用户批量软删除分享记录`
    async fn batch_soft_del_record(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频ID
        ids: Vec<i64>, // 目标IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
