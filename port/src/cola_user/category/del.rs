// port/src/cola_user/category/del.rs
// ⏩️ 端口 - USER -  分类 - 删除
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;

////////

/// # [ADD PORTS]
/// * `desc`: `用户分类删除端口`
#[async_trait]
pub trait UserCategoryDeletePort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 单个删除
    async fn single_delete(
        &self,
        id: i64, // 目标 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量删除
    async fn batch_delete(
        &self,
        ids: Vec<i64>, // 目标 IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
