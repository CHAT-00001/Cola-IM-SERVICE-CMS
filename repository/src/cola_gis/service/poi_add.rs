// repository/src/cola_gis/service/active  -- 仓储中心 - GIS - service - 兴趣点添加
// 2026/7/6 12:10

////////

use crate::cola_gis::pg::poi::PoiRepo;
use crate::cola_gis::pg::user::UserRepo;
use crate::cola_gis::pg::add::AddRepository;
use cola_data::cola_gis::command::poi::PoiCommand;
use cola_data::cola_gis::info::poi::PoiInfo;

////////

/// # [SERVICE] - 兴趣点 添加
pub struct PoiAddService;

// 构造实现
impl PoiAddService {
    //

    ////////

    /// # 1. [SERVICE] - ✅️ 保存新的兴趣点
    pub async fn save_poi_and_update_count(uid: i64, cmd: PoiCommand, visibility: i16) -> Result<PoiInfo, anyhow::Error> {
        let entity = PoiRepo::save_poi_by_uid(uid, cmd, visibility).await?;
        let async_uid = uid;
        tokio::spawn(async move {
            if let Err(e) = UserRepo::update_user_count(async_uid, 1, 0, 0, 0, 0, 0).await {
                tracing::error!("[🔌 ADAPTER]: 💾保存新的兴趣点失败! : uid={}, err={:?}", async_uid, e);
            }
        });
        Ok(PoiInfo::from_entity(entity))
    }

    ////////

    /// # 2. [SERVICE] - 🌬️ 修改兴趣点
    pub async fn edit_poi(uid: i64, cmd: PoiCommand, visibility: i16) -> Result<PoiInfo, anyhow::Error> {
        let entity = PoiRepo::save_poi_by_uid(uid, cmd, visibility).await?;
        Ok(PoiInfo::from_entity(entity))
    }

    /////////

    /// # 3. [SERVICE] - ❌️ 👤 用户软删除一个兴趣点(支持多个)
    pub async fn del_one_poi(poi_ids: Vec<i64>) -> Result<bool, anyhow::Error> {
        match AddRepository::pg_delete_poi_by_ids(poi_ids).await {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(e) => Err(anyhow::anyhow!("[SERVICE]: 👤 用户批量删除兴趣点POI 失败: {}", e)),
        }
    }
}


