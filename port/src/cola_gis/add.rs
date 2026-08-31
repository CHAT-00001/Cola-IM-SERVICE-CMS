// /port/active  -- 添加
// 2026/6/10 06:35

//////

use cola_data::cola_gis::command::poi::PoiCommand;

//////

/// # [PORT] - 添加
#[async_trait::async_trait]
pub trait AddPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 发布
    async fn add_poi(&self, uid: i64, data: PoiCommand) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn edit_poi(&self, uid: i64, poi_id: i64, data: PoiCommand) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 单个删除
    async fn del_one_poi(&self, uid: i64, poi_id: i64) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 多个删除
    async fn del_many_poi(&self, uid: i64, poi_ids: Vec<i64>) -> anyhow::Result<()>;
}
