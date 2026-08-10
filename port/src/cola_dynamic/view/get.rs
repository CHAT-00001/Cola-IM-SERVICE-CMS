// port/src/cola_dynamic/view/get.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 浏览 - 浏览 获取
// 2026/8/4 22:10 Created.

////////

////////

/// # [DYNAMIC VIEW PORTS]
/// * `desc`: `⏹ 可乐动态 - 动态浏览获取端口`
#[async_trait::async_trait]
pub trait DynamicViewGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 获取我的浏览记录IDs
    /// * `desc`: `用户批量获取浏览的视频IDs`
    async fn get_my_like_ids(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - 获取TA的浏览记录IDs
    /// * `desc`: `用户批量获取浏览的视频IDs`
    async fn get_he_like_ids(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
