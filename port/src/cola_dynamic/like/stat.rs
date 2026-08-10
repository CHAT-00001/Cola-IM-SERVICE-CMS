// like/stat.rs
// 动态 - port - 点赞 - 统计
// 2026/8/5 16:31 Created.

////////

/// # [STAT PORT] - 统计
/// * `desc`: `动态收藏统计端口`
#[async_trait::async_trait]
pub trait LikeStatPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 用户的
    /// * `desc`: `根据用户ID` - `统计购买数量`
    async fn stat_count_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # [PORT] - 动态的
    /// * `desc`: `根据动态ID` - `统计购买数量`
    async fn stat_count_by_dynamic_id(
        &self,
        uid: i64,
        dynami_id: i64, // 动态 ID
    ) -> anyhow::Result<(u64)>;
}

//////// END
