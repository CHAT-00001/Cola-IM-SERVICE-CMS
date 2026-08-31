// repo_adapter/src/cola_gis/collect.rs
// 🔌 适配器 - 可乐GIS - POI - 收藏
// 2026-07-07

////////

use async_trait::async_trait;
use cola_data::cola_gis::command::collect::PoiCollectCommand;
use port::cola_gis::collect::CollectRepo;
use repository::cola_gis::service::poi_collect::PoiCollectService;

////////

/// # [COLLECT ADAPRER] - 收藏
/// * `desc`: `GIS - POI 收藏适配器`
pub struct CollectPortAdapter;

#[async_trait]
impl CollectRepo for CollectPortAdapter {
    //

    ////////

    /// # 1. [PORT] - 保存收藏记录
    async fn save_collect_record(
        &self,
        uid: i64,
        poi_id: i64,
        cmd: PoiCollectCommand,
    ) -> anyhow::Result<()> {
        PoiCollectService::save_collect_and_update_count(uid, poi_id, &cmd).await?;
        Ok(())
    }

    ////////

    /// # 2. [PORT] - 删除收藏记录
    async fn del_collect_record(&self, uid: i64, poi_id: i64) -> anyhow::Result<()> {
        PoiCollectService::del_collect_and_update_count(uid, poi_id).await
    }

    ////////

    /// # 3. [PORT] - 根据用户ID获取收藏的POI IDs
    async fn get_collect_ids_by_user_id(
        &self,
        user_id: i64,
        keyword: Option<String>,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        PoiCollectService::find_collect_ids_by_user_id(user_id, keyword, offset, limit).await
    }
}

//////// END
