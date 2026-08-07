// cola_gis/src/case/feed.rs  -- GIS - 用例层 - FEED
// 2026-07-07

//////////

use crate::assembler::poi::build_poi_list_response;
use crate::model::vo::poi::PoiListResponse;
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use repository::cola_gis::service::home::PoiHomeService;
use repository::cola_gis::service::view::PoiViewService;
use repository::cola_gis::service::like::GisLikeService;

//////////

/// # [FEED CASE] - 数据流 用例
pub struct FeedCase;

impl FeedCase {
    //

    ////////

    /// # 1. [CASE] - 关注的人发布的兴趣点
    pub async fn case_feed_following(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<PoiListResponse> {
        let ids = ctx.user.following.get_following_ids(uid).await?;

        let infos =
            PoiViewService::batch_uids_get_gis_infos(ids, Some(url.keyword), url.offset, url.limit)
                .await
                .map_err(|e| {
                    anyhow::anyhow!("BIZ: 获取用户 {} 兴趣点列表失败: {}", url.user_id, e)
                })?;

        let resp =
            build_poi_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;

        Ok(resp)
    }

    ////////

    /// # 2. [CASE] - TA发布的兴趣点
    pub async fn case_feed_publish(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<PoiListResponse> {

        let infos = PoiViewService::get_gis_infos_by_uid(url.user_id, Some(url.keyword), url.offset, url.limit)
            .await
            .map_err(|e| {
                anyhow::anyhow!("BIZ: 获取用户 {} 发布的兴趣点列表失败: {}", url.user_id, e)
            })?;

        let resp =
            build_poi_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;

        Ok(resp)
    }

    ////////

    /// # 3. [CASE] - TA点赞的
    pub async fn case_feed_liked(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> anyhow::Result<PoiListResponse> {
        let ids = GisLikeService::get_user_like_ids(url.user_id, url.offset, url.limit)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 点赞的列表失败: {}", url.user_id, e))?;

        let infos = PoiViewService::batch_get_gis_infos(ids)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 点赞的列表失败: {}", url.user_id, e))?;

        let resp =
            build_poi_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;
        Ok(resp)
    }

    ////////

    /// # 4. [CASE] - TA附近的兴趣点
    pub async fn case_feed_nearby(
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
            .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 附近兴趣点失败: {}", url.user_id, e))?;

        let resp =
            build_poi_list_response(infos, Some(url.user_id), url.offset, url.limit, 0).await?;
        Ok(resp)
    }
}

///////// END