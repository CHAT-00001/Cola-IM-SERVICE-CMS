// port/src/cola_dynamic/like/del.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 点赞 - 删除
// 2026/8/5 01:56 Created.

////////

////////

/// # [DELETE PORTS] - 删除
/// `desc`: `可乐动态 - 点赞记录删除端口`
#[async_trait::async_trait]
pub trait LikeDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    /// * `desc`: `⏹ 可乐动态` - `根据ID单个软删除点赞记录`
    async fn single_soft_del_record(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
        like_id: i64,  // 点赞 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量删除
    /// * `desc`: `⏹ 可乐动态` - `根据IDs批量软删除点赞记录`
    async fn batch_soft_del_record(
        &self,
        uid: i64,           // UID
        video_id: i64,      // 视频 ID
        like_ids: Vec<i64>, // 点赞 IDs
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 3. [PORT] - 动态的
    /// * `desc`: `⏹ 可乐动态` - `根据IDs批量软删除点赞记录`
    /// * `condition`: `⚠️ AUTO` - `动态删除时` - 同步删除关联的点赞记录
    async fn delete_like_by_dynamic_id(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 4. [PORT] - 用户的
    /// * `desc`: `⏹ 可乐动态` - `根据IDs批量软删除点赞记录`
    /// * `condition`: `⚠️ AUTO` - `用户注销时` - 同步删除他的点赞记录
    async fn delete_like_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u16)>;
}

//////// END
