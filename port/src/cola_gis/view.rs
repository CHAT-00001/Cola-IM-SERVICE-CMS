// port/src/cola_gis/poi/get.rs
// ⏩️ 端口 - 可乐GIS - POI - 浏览
// 2026/7/7 14:10 Created.

////////

use cola_data::cola_gis::info::poi::PoiInfo;

////////

/// # [GET PORTS] - 浏览 服务
#[async_trait::async_trait]
pub trait ViewPort: Send + Sync {

    ////////

    /// # [PORT] - 保存浏览记录 + 更新浏览数量
    async fn save_view_record_update_views_count(
        &self,
        uid: i64,
        poi_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 报告浏览完成（完播） + 更新完播数量
    async fn view_done_update_done_count(
        &self,
        uid: i64,
        poi_id: i64,
        is_done: bool,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 单个获取兴趣点
    async fn get_poi_list_by_id(
        &self,
        poi_id: i64,
    ) -> anyhow::Result<PoiInfo>;

    ////////

    /// # [PORT] - 批量获取兴趣点
    async fn get_poi_list_by_ids(
        &self,
        poi_ids: Vec<i64>,
    ) -> anyhow::Result<Vec<PoiInfo>>;
}