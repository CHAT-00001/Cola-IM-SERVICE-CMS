// repo/src/gis/service/home.rs  -- 仓储中心 - GIS - service - 主页
// 2026/7/6 20:14

////////

use crate::gis::pg::poi::PoiRepo;
use crate::gis::pg::user::UserRepo;
use cola_data::gis::command::collect::PoiCollectCommand;
use cola_data::gis::entity::collect::PoiCollectEntity;
use cola_data::gis::info::poi::PoiInfo;

////////

/// # [💁 SERVICE] - 兴趣点 主页
pub struct PoiHomeService;

// 构造实现
impl PoiHomeService {
    //

    ////////

    /// # 1. [🔌 ADAPTER] - ▶ ☀️ 最新
    pub async fn find_new_gis_list(limit: i64, offset: i64) -> Result<Vec<PoiInfo>, anyhow::Error> {
        let entities = PoiRepo::find_new_list(limit, offset).await?;
        Ok(entities.into_iter().map(PoiInfo::from_entity).collect())
    }

    ////////

    /// # 2. [🔌 ADAPTER] - ▶ 🔥 热门
    pub async fn find_hot_gis_list(limit: i64, offset: i64) -> Result<Vec<PoiInfo>, anyhow::Error> {
        let entities = PoiRepo::find_hot_list(limit, offset).await?;
        Ok(entities.into_iter().map(PoiInfo::from_entity).collect())
    }

    ////////

    /// # 3. [🔌 ADAPTER] - ▶ ⚙️ 随机
    pub async fn find_recommend_gis_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PoiInfo>, anyhow::Error> {
        let entities = PoiRepo::find_recommend_list(limit, offset).await?;
        Ok(entities.into_iter().map(PoiInfo::from_entity).collect())
    }

    ////////

    /// # 4. [🔌 ADAPTER] - ▶ 🏙️ 同城
    pub async fn find_city_gis_list(
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PoiInfo>, anyhow::Error> {
        let entities = PoiRepo::find_nearby_list(lat, lng, limit, offset).await?;
        Ok(entities.into_iter().map(PoiInfo::from_entity).collect())
    }

    ////////

    /// # 5. [🔌 ADAPTER] - ▶ ⭐ 精选
    pub async fn find_featured_gis_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PoiInfo>, anyhow::Error> {
        let entities = PoiRepo::find_featured_list(limit, offset).await?;
        Ok(entities.into_iter().map(PoiInfo::from_entity).collect())
    }

    ////////

    /// # 6. [🔌 ADAPTER] - ▶  🔍 搜索
    pub async fn search_gis_keyword_list(
        keyword: String,
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PoiInfo>, anyhow::Error> {
        let entities =
            PoiRepo::search_keyword_list(keyword, lat, lng, None, None, None, limit, offset)
                .await?;
        Ok(entities.into_iter().map(PoiInfo::from_entity).collect())
    }

    ////////

    /// # 7. [🔌 ADAPTER] - ▶ 👤 用户
    pub async fn find_gis_by_user_ids(
        uids: Vec<i64>,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PoiInfo>, anyhow::Error> {
        let entities = PoiRepo::find_list_by_uids(Some(uids), keyword, limit, offset).await?;
        Ok(entities.into_iter().map(PoiInfo::from_entity).collect())
    }

    ////////

    /// # 8. [🔌 ADAPTER] - ▶ ❤️ 收藏
    pub async fn save_collect_and_update_count(
        uid: i64,
        _gis_id: i64,
        _cmd: PoiCollectCommand,
    ) -> Result<PoiCollectEntity, anyhow::Error> {
        let entity = PoiCollectEntity::default();
        let async_uid = uid;
        tokio::spawn(async move {
            if let Err(e) = UserRepo::update_user_count(async_uid, 0, 0, 0, 1, 0, 0).await {
                tracing::error!(
                    "SERVICE_ASYNC: GIS鏀惰棌璁℃暟鏇存柊澶辫触: uid={}, err={:?}",
                    async_uid,
                    e
                );
            }
        });
        Ok(entity)
    }
}

//////// END
