// service/src/cola_gis/danmaku/poi_danmaku.rs
// 服务 - 可乐GIS - 弹幕 - 兴趣点 弹幕
// 2026/7/6 12:01

////////

use cola_data::cola_gis::command::danmaku::PoiDanmakuCommand;
use cola_data::cola_gis::entity::danmaku::PoiDanmakuEntity;
use cola_data::cola_gis::info::danmaku::PoiDanmakuInfo;
use repository::cola_gis::pg::poi_danmaku::PoiDanmakuRepo;
use repository::cola_gis::pg::user::UserRepo;

////////

/// # [DANMAKU SERVICE] - 弹幕
/// * `desc`: `POI弹幕服务`
pub struct GisDanmakuService;

// 构造实现
impl GisDanmakuService {
    //

    ////////

    /// # 1. [🔌 ADAPTER] - ✅️ 保存弹幕记录
    pub async fn save_danmaku_and_update_count(
        uid: i64,
        gis_id: i64,
        cmd: PoiDanmakuCommand,
        visibility: i16,
    ) -> Result<PoiDanmakuEntity, anyhow::Error> {
        let entity = PoiDanmakuRepo::save_danmaku_by_gis_id(uid, gis_id, cmd, visibility).await?;
        let async_uid = uid;
        tokio::spawn(async move {
            if let Err(e) = UserRepo::update_user_count(async_uid, 1, 0, 0, 0, 0, 0).await {
                tracing::error!(
                    "[🔌 ADAPTER]: 保存📍兴趣点弹幕失败: uid={}, err={:?}",
                    async_uid,
                    e
                );
            }
        });
        Ok(entity)
    }

    ////////

    /// # 2. [🔌 ADAPTER] - ▶ 📍 根据兴趣点ID查找弹幕
    pub async fn get_poi_danmaku(
        gis_id: i64,
        play_time: i32,
        time_window: i32,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<PoiDanmakuInfo>, anyhow::Error> {
        let entities =
            PoiDanmakuRepo::find_danmaku_by_gis_id(gis_id, play_time, time_window, limit, offset)
                .await?;
        Ok(entities
            .into_iter()
            .map(PoiDanmakuInfo::from_entity)
            .collect())
    }

    pub async fn delete_danmaku_and_update_count(
        uid: i64,
        danmaku_id: i64,
    ) -> Result<(), anyhow::Error> {
        // For now, just delete
        PoiDanmakuRepo::user_del_danmaku_by_gis_id(danmaku_id).await?;
        Ok(())
    }
}

//////// END
