// port/src/cola_video/port/share/stat.rs
// ⏩️ 端口 - 可乐动态 - 分享 - 统计
// 2026/8/5 00:01 Created.

////////

////////

/// # [DEL SERVICE] - 统计
/// `desc`: `视频分享统计服务端口`
#[async_trait::async_trait]
pub trait DynamicShareStatPort: Send + Sync {
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
