// port/src/cola_video/video/del.rs
// ⏩️ 端口 - 可乐视频 -  视频 - 删除
// 2026/8/5 00:00 Created.

////////

////////

/// # [DELETE PORTS] - 软删除
/// * `desc`: `视频删除端口`
#[async_trait::async_trait]
pub trait VideoDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    /// * `desc`: `根据视频ID - 批量删除`
    async fn single_soft_del(
        &self,
        uid: i64, // UID
        video_id: i64,  // 视频 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量软删除
    /// * `desc`: `根据视频IDs - 批量删除`
    async fn batch_soft_del(
        &self,
        uid: i64,      // UID
        video_ids: Vec<i64>, // 视频 IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
