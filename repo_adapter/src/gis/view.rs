// repo_adapter/src/gis/view.rs
// 2026-07-07

//////

use async_trait::async_trait;
use cola_data::gis::info::poi::PoiInfo;
use cola_data::gis::port::view::ViewPort;
use repo::gis::service::view::PoiViewService;

//////

/// # [VIEW PORT] - 浏览 端口 插头
pub struct ViewPortAdapter;

//////

#[async_trait]
impl ViewPort for ViewPortAdapter {
    ////////

    /// # 1. 保存浏览记录 + 更新浏览数量
    async fn save_view_record_update_views_count(
        &self,
        _uid: i64,
        _poi_id: i64,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # 2. 浏览完成报告 + 更新完播数量
    async fn view_done_update_done_count(
        &self,
        _uid: i64,
        _poi_id: i64,
        _is_done: bool,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # 3. [PORT] - 获取一个兴趣点信息
    async fn get_poi_list_by_id(&self, poi_id: i64) -> anyhow::Result<PoiInfo> {
        let mut infos = PoiViewService::batch_get_gis_infos(vec![poi_id]).await?;
        infos.pop().ok_or_else(|| anyhow::anyhow!("POI not found: {}", poi_id))
    }

    ////////

    /// # 4. [PORT] - 遍历兴趣点ids获取信息
    async fn get_poi_list_by_ids(&self, poi_ids: Vec<i64>) -> anyhow::Result<Vec<PoiInfo>> {
        PoiViewService::batch_get_gis_infos(poi_ids).await
    }
}

////// END