// /port/add.rs  -- 添加
// 2026/6/10 06:35

////////

use crate::dynamic::command::dynamic::DynamicCommand;

////////


/// # [SERVICE PORT] - 添加
#[async_trait::async_trait]
pub trait AddPort: Send + Sync {

    ////////

    /// # 1. [PORT] - 发布
    async fn add_video(
        &self,
        uid: i64,
        data: DynamicCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn edit_video(
        &self,
        uid: i64,
        dynamic_id: i64,
        data: DynamicCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 单个删除
    async fn single_delete(
        &self,
        uid: i64,
        dynamic_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 批量删除
    async fn batch_delete(
        &self,
        uid: i64,
        dynamic_ids: Vec<i64>,
    ) -> anyhow::Result<()>;
}