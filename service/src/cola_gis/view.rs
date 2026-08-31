// repository/src/cola_gis/service/view.rs
// 服务 - 可乐GIS - 浏览 - 模块
// 2026/7/6

////////

use cola_data::cola_gis::info::poi::PoiInfo;
use repository::cola_gis::pg::view::GisViewRepo;
////////

/// # [SERVICE] - 兴趣点 浏览 服务
pub struct PoiViewService;

// 构造实现
impl PoiViewService {
    //

    ////////

    /// # 1. [🔌 ADAPTER] - 批量查找兴趣点
    pub async fn batch_get_gis_infos(ids: Vec<i64>) -> Result<Vec<PoiInfo>, anyhow::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let entities = GisViewRepo::find_all_batch_ids(&ids).await?;
        Ok(entities.into_iter().map(PoiInfo::from_entity).collect())
    }

    ////////

    /// # 2. [🔌 ADAPTER] - 遍历用户ID查找兴趣点
    pub async fn batch_uids_get_gis_infos(
        uids: Vec<i64>,
        keyword: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<PoiInfo>, anyhow::Error> {
        if uids.is_empty() {
            return Ok(vec![]);
        }
        let entities = GisViewRepo::pg_batch_uids_find_list(uids, keyword, offset, limit).await?;
        Ok(entities.into_iter().map(PoiInfo::from_entity).collect())
    }

    ////////

    /// # 3. [🔌 ADAPTER] - 根据用户ID查找兴趣点
    pub async fn get_gis_infos_by_uid(
        user_id: i64,
        keyword: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<PoiInfo>, anyhow::Error> {
        let entities =
            GisViewRepo::pg_find_new_list_by_uid(user_id, keyword, offset, limit).await?;
        Ok(entities.into_iter().map(PoiInfo::from_entity).collect())
    }
}

//////// END
