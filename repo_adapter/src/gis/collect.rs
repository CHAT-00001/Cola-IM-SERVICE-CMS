// repo_adapter/src/gis/collect.rs
// 2026-07-07

use async_trait::async_trait;
use cola_data::gis::port::collect::CollectRepo;
use cola_data::gis::command::collect::PoiCollectCommand;
use repo::gis::service::poi_collect::PoiCollectService;

pub struct CollectPortAdapter;

#[async_trait]
impl CollectRepo for CollectPortAdapter {

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

    /// # 2. [PORT] - 删除收藏记录
    async fn del_collect_record(
        &self,
        uid: i64,
        poi_id: i64,
    ) -> anyhow::Result<()> {
        PoiCollectService::del_collect_and_update_count(uid, poi_id).await
    }

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