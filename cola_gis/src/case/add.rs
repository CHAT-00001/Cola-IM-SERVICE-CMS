// cola_gis/src/case/poi_add.rs  -- GIS - case - 兴趣点发布
// 2026-06-11 04:12

////////

use anyhow::{Context, Result};
use tracing::info;
use cola_data::fs::rick_check;
use cola_data::gis::command::poi::PoiCommand;
use repository::gis::service::poi_add::PoiAddService;
use crate::assembler::poi::build_poi_single_response;
use crate::model::vo::poi::PoiSingleResponse;

////////

/// # [CASE] - 兴趣点 添加 用例
pub struct PoiAddCase;

// 构造实现
impl PoiAddCase {

    ////////

    /// # 1. [CASE] - 发布
    pub async fn case_add_publish(uid: i64, cmd: PoiCommand) -> Result<PoiSingleResponse, anyhow::Error> {
        // 1. 内容风控（标题 + 简介 联合过滤）
        let check_text = format!("{} {:?}", cmd.title, cmd.description);

        // ✅ 核心修复：rick_check 异步执行后出来就是 i16，直接 await 拿值，删掉多余的 map_err!?
        let visibility = rick_check(check_text).await;

        // 2. 核心数据持久化与计数更新 (💡 提示：建议让这个 Service 函数返回刚插入成功的 PoiInfo)
        let poi_info = PoiAddService::save_poi_and_update_count(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: POI 发布持久化失败: {}", e))?;

        info!("BIZ - POI 发布成功: uid={}, visibility={}", uid, visibility);

        // 3. 🌟 架构对齐：用我们刚才写好的高质量总装器，动态拼装博主信息后返回给前端
        let response = build_poi_single_response(poi_info, Some(uid)).await?;

        Ok(response)
    }

    ////////

    /// # 2. [CASE] - 编辑
    pub async fn case_edit_publish(uid: i64, cmd: PoiCommand) -> Result<PoiSingleResponse, anyhow::Error> {
        // 1. 内容风控（标题 + 简介 联合过滤）
        let check_text = format!("{} {:?}", cmd.title, cmd.description);

        // ✅ 核心修复：同上，直接接住 i16
        let visibility = rick_check(check_text).await;

        // 2. 核心数据持久化与计数更新
        let poi_info = PoiAddService::edit_poi(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: POI 发布持久化失败: {}", e))?;

        info!("BIZ - POI 发布成功: uid={}, visibility={}", uid, visibility);

        // 3. 🌟 架构对齐：用我们刚才写好的高质量总装器，动态拼装博主信息后返回给前端
        let response = build_poi_single_response(poi_info, Some(uid)).await?;

        Ok(response)
    }

    ////////
}

//////// END