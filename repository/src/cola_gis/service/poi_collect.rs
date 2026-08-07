// repository/src/cola_gis/service/poi_collect.rs -- 仓储中心 - GIS - service - 兴趣点 收藏
// 2026/7/6

////////

use crate::cola_gis::pg::poi_collect::PoiCollectRepo;
use crate::cola_gis::pg::count::CountRepo;
use crate::cola_gis::pg::user::UserRepo;
use cola_data::cola_gis::command::collect::PoiCollectCommand;
use cola_data::cola_gis::entity::collect::PoiCollectEntity;

////////

pub struct PoiCollectService;

impl PoiCollectService {
    // BUILD

    ////////

    /// # 1. [SERVICE] - ✅️ 保存兴趣点收藏记录
    /// * `params`: `cmd`
    pub async fn save_collect_and_update_count(uid: i64, gis_id: i64, cmd: &PoiCollectCommand) -> Result<(), anyhow::Error> {
        PoiCollectRepo::save_collect_by_gis_id(uid, gis_id, cmd).await?;
        let async_uid = uid;
        tokio::spawn(async move {
            if let Err(e) = UserRepo::update_user_count(async_uid, 0, 0, 0, 1, 0, 0).await {
                tracing::error!("[SERVICE]: Poi 兴趣点 保存失败: uid={}, err={:?}", async_uid, e);
            }
        });
        Ok(())
    }

    ////////

    /// # 2. [SERVICE] - ❌️ 删除兴趣点收藏记录
    /// * `params`: `cmd`
    pub async fn del_collect_and_update_count(uid: i64, gis_id: i64) -> Result<(), anyhow::Error> {
        PoiCollectRepo::delete_collect_by_gis_id(uid, gis_id).await?;
        CountRepo::pg_update_gis_collects(gis_id, -1).await?;
        Ok(())
    }

    ////////

    /// # 3. [SERVICE] - ▶ 👤  根据用户ID查找TA的收藏记录
    /// * `params`: `cmd`
    pub async fn find_collect_ids_by_user_id(user_id: i64, keyword: Option<String>, offset: i64, limit: i64) -> Result<Vec<i64>, anyhow::Error> {
        let ids = PoiCollectRepo::find_collect_ids_by_user_id(user_id, keyword, limit, offset).await?;
        Ok(ids)
    }
}

//////// END
