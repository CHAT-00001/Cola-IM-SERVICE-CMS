// port/src/cola_video/view/delete.rs
// ⏩️ 端口 - MARKET - 地址簿 - 删除端口
// 2026/8/4 22:10 Created.

////////

/// # [DELETE PORTS]
/// * `desc`: `VIDEO - 视频浏览记录删除端口`
#[async_trait::async_trait]
pub trait AddressDeletePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个删除
    async fn single_delete(
        &self,
        view_id: i64, // 浏览 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量删除
    async fn batch_delete(
        &self,

        view_ids: Vec<i64>, // 浏览 IDs
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 3. [PORT] - 用户删除时
    async fn delete_address_by_user_id(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)>;
}

//////// END
