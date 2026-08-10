// port/src/cola_video/view/del.rs
// ⏩️ 端口 - ▶ 可乐视频 - 浏览 - 删除
// 2026/8/4 22:10 Created.

////////

////////

/// # [DEL PORTS]
/// * `desc`: `视频浏览删除端口`
#[async_trait::async_trait]
pub trait VideoViewDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    /// * `desc`: `⏹ 可乐动态` - `根据ID单个软删除不喜欢记录`
    async fn single_soft_del_record(
        &self,
        uid: i64,        // UID
        video_id: i64,   // 视频 ID
        dislike_id: i64, // 不喜欢 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量删除
    /// * `desc`: `⏹ 可乐动态` - `根据IDs批量软删除不喜欢记录`
    async fn batch_soft_del_record(
        &self,
        uid: i64,              // UID
        video_id: i64,         // 视频 ID
        dislike_ids: Vec<i64>, // 不喜欢 IDs
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 3. [PORT] - 动态的
    /// * `desc`: `⏹ 可乐动态` - `根据IDs批量软删除不喜欢记录`
    /// * `condition`: `⚠️ AUTO` - `动态删除时` - 同步删除关联的不喜欢记录
    async fn delete_dislike_by_dynamic_id(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 4. [PORT] - 用户的
    /// * `desc`: `⏹ 可乐动态` - `根据IDs批量软删除不喜欢记录`
    /// * `condition`: `⚠️ AUTO` - `用户注销时` - 同步删除他的不喜欢记录
    async fn delete_dislike_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u16)>;
}

//////// END
