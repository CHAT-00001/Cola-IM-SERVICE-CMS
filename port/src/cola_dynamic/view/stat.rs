// port/src/cola_dynamic/view/stat.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 浏览 - 统计
// 2026/8/4 22:11 Created.

////////

/// # [STAT PORTS] 浏览 统计
/// * `desc`: `动态浏览统计端口`
#[async_trait::async_trait]
pub trait DynamicViewStatPort:  Send + Sync {
    //

    ////////

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
