// cola_gis/src/case/get  -- 可乐GIS - 用例层 - 浏览
// 2026-07-07

//////

use crate::assembler::poi::build_poi_single_response;
use crate::model::vo::poi::PoiSingleResponse;
use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_gis::info::poi::PoiInfo;

//////

/// # [CASE] - 浏览 用例
pub struct PoiViewCase;

impl PoiViewCase {
    //
    ////////

    /// # [CASE] - 保存浏览记录
    pub async fn case_add_poi_view(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext) -> Result<()> {

        // Call Service Port
        ctx.gis
            .view
            .save_view_record_update_views_count(uid, url.video_id)
            .await
            .map_err(|e| anyhow!("保存浏览记录 + 更新浏览数量失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # [CASE] - 获取兴趣点详情
    pub async fn case_get_poi_detail(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<PoiSingleResponse> {

        // Call Service Port
        let info: PoiInfo = ctx
            .gis
            .view
            .get_poi_list_by_id(url.video_id)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 查询POI详情失败: {}", e))?;

        let resp = build_poi_single_response(info, Some(uid)).await?;
        Ok(resp)
    }
}

////// END