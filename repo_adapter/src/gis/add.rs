// repo_adapter/src/cola_gis/active -- 适配器 - GIS - 添加
// 2026-07-07 10:12

////////

use async_trait::async_trait;
use cola_data::cola_gis::command::poi::PoiCommand;
use cola_data::cola_gis::port::add::AddPort;
use repository::cola_gis::service::poi_add::PoiAddService;

////////

/// # [ADD PORT] - 添加 端口 插头
pub struct AddPortAdapter;

////////

#[async_trait]
impl AddPort for AddPortAdapter {
    // 💡

    ////////

    /// # 1. [PORT] - 保存兴趣点记录 + 更新用户兴趣点数量
    async fn add_poi(&self, uid: i64, data: PoiCommand) -> anyhow::Result<()> {
        PoiAddService::save_poi_and_update_count(uid, data, 1).await?;
        Ok(())
    }

    ////////

    /// # 2. [PORT] - 编辑兴趣点
    async fn edit_poi(&self, uid: i64, _poi_id: i64, data: PoiCommand) -> anyhow::Result<()> {
        PoiAddService::edit_poi(uid, data, 1).await?;
        Ok(())
    }

    ////////

    /// # 3. [PORT] - 删除单个兴趣点
    async fn del_one_poi(&self, _uid: i64, poi_id: i64) -> anyhow::Result<()> {
        PoiAddService::del_one_poi(vec![poi_id]).await?;
        Ok(())
    }

    ////////

    /// # 4. [PORT] - 遍历兴趣点IDs批量删除兴趣点
    async fn del_many_poi(&self, _uid: i64, poi_ids: Vec<i64>) -> anyhow::Result<()> {
        PoiAddService::del_one_poi(poi_ids).await?;
        Ok(())
    }
}

//////// END
