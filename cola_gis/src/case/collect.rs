// cola_gis/src/case/count  -- GIS - 用例层 - 收藏
// 2026-07-07 10:10

////////

use anyhow::Result;
use tracing::{info, warn};
use cola_data::gis::command::collect::PoiCollectCommand;
use repository::gis::service::poi_collect::PoiCollectService;

////////

/// # [USE CASE] - 收藏 用例
pub struct CollectCase;

impl CollectCase {

    ////////

    /// # 1. [CASE] - 添加
    pub async fn case_add_collect(
        uid: i64,
        poi_id: i64,
        cmd: PoiCollectCommand,
    ) -> Result<bool> {

        PoiCollectService::save_collect_and_update_count(uid, poi_id, &cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 添加收藏失败: {}", e))?;

        info!("BIZ - 添加收藏成功: uid={}, poi_id={}", uid, poi_id);
        Ok(true)
    }

    ////////

    /// # 2. [CASE] - 删除
    pub async fn case_del_collect(
        uid: i64,
        poi_id: i64,
    ) -> Result<bool> {

        PoiCollectService::del_collect_and_update_count(uid, poi_id)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 删除收藏失败: {}", e))?;

        info!("BIZ - 删除收藏成功: uid={}, poi_id={}", uid, poi_id);
        Ok(true)
    }

    ////////
}

//////// END