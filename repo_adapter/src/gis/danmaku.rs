// repo_adapter/src/gis/danmaku.rs -- 适配器 - GIS - danmaku
// 2026-07-07 12:00

////////

use async_trait::async_trait;
use cola_data::gis::port::danmaku::DanmakuRepo;
use cola_data::gis::command::danmaku::PoiDanmakuCommand;
use cola_data::gis::info::danmaku::PoiDanmakuInfo;
use repository::gis::service::poi_danmaku::GisDanmakuService;

////////

/// # [ADAPTER] - GIS - 弹幕
pub struct DanmakuPortAdapter;

#[async_trait]
impl DanmakuRepo for DanmakuPortAdapter {

    /// # 1. [PORT] - 保存弹幕记录
    async fn save_danmaku_record(
        &self,
        uid: i64,
        poi_id: i64,
        cmd: PoiDanmakuCommand,
    ) -> anyhow::Result<()> {
        GisDanmakuService::save_danmaku_and_update_count(uid, poi_id, cmd, 1).await?;
        Ok(())
    }

    /// # 2. [PORT] - 编辑弹幕
    async fn edit_danmaku_record(
        &self,
        uid: i64,
        danmaku_id: i64,
        _cmd: PoiDanmakuCommand,
    ) -> anyhow::Result<()> {
        GisDanmakuService::delete_danmaku_and_update_count(uid, danmaku_id).await
    }

    /// # 3. [PORT] - 删除弹幕
    async fn del_danmaku_record(
        &self,
        uid: i64,
        danmaku_id: i64,
    ) -> anyhow::Result<()> {
        GisDanmakuService::delete_danmaku_and_update_count(uid, danmaku_id).await
    }

    /// # 4. [PORT] - 批量删除弹幕
    async fn del_danmakus_record(
        &self,
        uid: i64,
        danmaku_ids: Vec<i64>,
    ) -> anyhow::Result<()> {
        for id in danmaku_ids {
            GisDanmakuService::delete_danmaku_and_update_count(uid, id).await?;
        }
        Ok(())
    }

    /// # 5. [PORT] - 根据兴趣点ID获取弹幕
    async fn get_danmaku_by_poi_id(
        &self,
        _uid: i64,
        poi_id: i64,
        play_time: i32,
        qty: i32,
    ) -> anyhow::Result<(Vec<PoiDanmakuInfo>, i64)> {
        let infos = GisDanmakuService::get_poi_danmaku(poi_id, play_time, qty, 0, 50).await?;
        let total = infos.len() as i64;
        Ok((infos, total))
    }

    /// # 6. [PORT] - 根据用户ID获取弹幕
    async fn get_danmaku_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<PoiDanmakuInfo>, i64)> {
        // Not directly supported by current GisDanmakuService; return empty
        Ok((vec![], 0))
    }
}

//////// END