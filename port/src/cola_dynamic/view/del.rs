// port/src/cola_dynamic/view/del.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 浏览 - 删除
// 2026/8/4 22:10 Created.

////////

////////

/// # [VIDEO VIEW DEL PORT]
/// * `desc`: `动态浏览删除端口`
#[async_trait::async_trait]
pub trait DynamicViewDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    /// * `desc`: `⏹ 可乐动态` - `根据ID单个软删除浏览记录`
    async fn single_soft_del_record(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
        like_id: i64,    // 浏览 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量删除
    /// * `desc`: `⏹ 可乐动态` - `根据IDs批量软删除浏览记录`
    async fn batch_soft_del_record(
        &self,
        uid: i64,           // UID
        dynamic_id: i64,    // 动态 ID
        like_ids: Vec<i64>, // 浏览 IDs
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 3. [PORT] - 动态的
    /// * `desc`: `⏹ 可乐动态` - `根据IDs批量软删除浏览记录`
    /// * `condition`: `⚠️ AUTO` - `动态删除时` - 同步删除关联的浏览记录
    async fn delete_like_by_dynamic_id(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 4. [PORT] - 用户的
    /// * `desc`: `⏹ 可乐动态` - `根据IDs批量软删除浏览记录`
    /// * `condition`: `⚠️ AUTO` - `用户注销时` - 同步删除他的浏览记录
    async fn delete_like_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u16)>;
}

//////// END
