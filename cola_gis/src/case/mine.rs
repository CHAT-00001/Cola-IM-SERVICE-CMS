// cola_gis/src/case/mine.rs  -- 可乐GIS - 用例层 - 我的
// 2026-07-07 12:00

////////

use crate::assembler::poi::build_poi_list_response;
use crate::model::vo::poi::PoiListResponse;
use anyhow::{Context, Result};
use cola_data::app::query::ApiGatewayRequest;
use port::app::ctx::AppContext;
use repository::cola_gis::service::like::GisLikeService;
use repository::cola_gis::service::view::PoiViewService;

////////

/// # [MINE CASE]
pub struct MineCase;

impl MineCase {
    //

    ////////

    /// # 1. [CASE] - 我发布的兴趣点
    pub async fn case_mine_publish(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<PoiListResponse> {
        let infos =
            PoiViewService::get_gis_infos_by_uid(uid, Some(url.keyword), url.offset, url.limit)
                .await
                .map_err(|e| {
                    anyhow::anyhow!("BIZ: 获取用户 {} 发布的兴趣点列表失败: {}", uid, e)
                })?;

        let resp = build_poi_list_response(infos, Some(uid), url.offset, url.limit, 0).await?;

        Ok(resp)
    }

    ////////

    /// # 2. [CASE] - 我点赞的兴趣点
    pub async fn case_mine_liked(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<PoiListResponse> {
        let ids = GisLikeService::get_user_like_ids(uid, url.offset, url.limit)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 点赞的列表失败: {}", uid, e))?;

        let infos = PoiViewService::batch_get_gis_infos(ids)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 点赞的兴趣点失败: {}", uid, e))?;

        let resp = build_poi_list_response(infos, Some(uid), url.offset, url.limit, 0).await?;
        Ok(resp)
    }

    ////////

    /// # 3. [CASE] - 附近的兴趣点
    pub async fn case_mine_nearby(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<PoiListResponse> {
        let lat = url.lat.unwrap_or(0.0);
        let lng = url.lng.unwrap_or(0.0);
        let range = 50000.0;

        let infos = ctx
            .gis
            .feed
            .get_nearby_list(lat, lng, range, url.offset, url.limit)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取附近兴趣点失败: {}", e))?;

        let resp = build_poi_list_response(infos, Some(uid), url.offset, url.limit, 0).await?;
        Ok(resp)
    }
}

//////// END
