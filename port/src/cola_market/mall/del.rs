// port/src/cola_video/video/del.rs
// ⏩️ 端口 - VIDEO -  视频 - 删除
// 2026/8/5 00:00 Created.

////////

/// # [DELETE PORTS] - 软删除
/// * `desc`: `VIDEO - 视频记录删除端口`
#[async_trait::async_trait]
pub trait VideoDeletePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个删除
    async fn single_delete(
        &self,
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # 2. [PORT] - 批量删除
    async fn batch_delete(
        &self,
        video_ids: Vec<i64>, // 视频 IDs
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # 3. [PORT] - 用户删除时
    async fn delete_videos_by_user_id(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)>;
}

//////// END
