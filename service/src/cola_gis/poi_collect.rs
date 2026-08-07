// servicey/src/cola_gis/poi/poi_collect.rs --
// 服务 - 可乐GIS - 兴趣点 -  收藏
// 2026/7/6 Created.

////////

use cola_data::cola_gis::command::collect::PoiCollectCommand;
use cola_data::cola_gis::entity::collect::PoiCollectEntity;
use repository::cola_gis::pg::count::CountRepo;
use repository::cola_gis::pg::poi_collect::PoiCollectRepo;
use repository::cola_gis::pg::user::UserRepo;

////////

/// # [COLLECT SERVICE] - 收藏服务
/// * `desc`: `兴趣点收藏服务`
pub struct PoiCollectService;

impl PoiCollectService {
    // BUILD

    ////////

    /// # 1. [SERVICE] - ✅️ 保存兴趣点收藏记录
    /// * `params`: `cmd`
    pub async fn save_collect_and_update_count(
        uid: i64,    // 用户ID
        gis_id: i64, // POI ID
        cmd: &PoiCollectCommand,
    ) -> Result<(), anyhow::Error> {
        PoiCollectRepo::save_collect_by_gis_id(uid, gis_id, cmd).await?;
        let async_uid = uid;
        tokio::spawn(async move {
            if let Err(e) = UserRepo::update_user_count(async_uid, 0, 0, 0, 1, 0, 0).await {
                tracing::error!(
                    "[SERVICE]: Poi 兴趣点 保存失败: uid={}, err={:?}",
                    async_uid,
                    e
                );
            }
        });
        Ok(())
    }

    ////////

    /// # 2. [SERVICE] - ❌️ 删除兴趣点收藏记录
    /// * `params`: `cmd`
    pub async fn del_collect_and_update_count(
        uid: i64,    // 用户 ID
        gis_id: i64, // POI ID
    ) -> Result<(), anyhow::Error> {
        PoiCollectRepo::delete_collect_by_gis_id(uid, gis_id).await?;
        CountRepo::pg_update_gis_collects(gis_id, -1).await?;
        Ok(())
    }

    ////////

    /// # 3. [SERVICE] - ▶ 👤  根据用户ID查找TA的收藏记录
    /// * `params`: `cmd`
    pub async fn find_collect_ids_by_user_id(
        user_id: i64,            // 用户 ID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> Result<Vec<i64>, anyhow::Error> {
        let ids =
            PoiCollectRepo::find_collect_ids_by_user_id(user_id, keyword, limit, offset).await?;
        Ok(ids)
    }
}

//////// END
