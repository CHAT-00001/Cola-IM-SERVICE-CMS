// cola_gis/src/case/home2  -- GIS - 用例层 - home
// 2026-07-07 08:41

////////

use crate::assembler::poi::build_poi_list_response;
use crate::model::vo::poi::PoiListResponse;
use anyhow::Result;
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::user::info::config::UserConfigInfo;
use repository::gis::service::home::PoiHomeService;

////////

/// # [HOME CASE] - 主页 用例
pub struct HomeCase;

impl HomeCase {
    //

    ////////

    /// # 1. [CASE] - 配置
    pub async fn case_get_con(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<UserConfigInfo> {
        let entities = ctx.user.con.get_config(uid).await?;
        Ok(entities)
    }

    ////////

    /// # 2. [CASE] - 新的
    pub async fn case_get_new_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<PoiListResponse> {
        let poi_infos = PoiHomeService::find_new_gis_list(url.limit, url.offset).await?;

        let response = build_poi_list_response(
            poi_infos,
            Some(uid),
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
        )
            .await?;

        Ok(response)
    }

    ////////

    /// # 3. [CASE] - 热门
    pub async fn case_get_hot_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<PoiListResponse> {
        let poi_infos = PoiHomeService::find_hot_gis_list(url.limit, url.offset).await?;

        let response =
            build_poi_list_response(poi_infos, url.uid, url.page.unwrap_or(1), url.qty.unwrap_or(10), 0).await?;

        Ok(response)
    }

    ////////

    /// # 4. [CASE] - 推荐
    pub async fn case_get_recommend_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<PoiListResponse> {
        let poi_infos =
            PoiHomeService::find_recommend_gis_list(url.limit, url.offset).await?;

        let response =
            build_poi_list_response(poi_infos, url.uid, url.page.unwrap_or(1), url.qty.unwrap_or(10), 0).await?;

        Ok(response)
    }

    ////////

    /// # 5. [CASE] - 同城
    pub async fn case_get_city_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<PoiListResponse> {
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        let poi_infos = PoiHomeService::find_city_gis_list(lat, lng, url.limit, url.offset).await?;

        let response =
            build_poi_list_response(poi_infos, url.uid, url.page.unwrap_or(1), url.qty.unwrap_or(10), 0).await?;

        Ok(response)
    }

    /// # 6. [CASE] - 分类
    pub async fn case_get_category_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<PoiListResponse> {
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        let poi_infos = PoiHomeService::find_city_gis_list(lat, lng, url.limit, url.offset).await?;

        let response =
            build_poi_list_response(poi_infos, url.uid, url.page.unwrap_or(1), url.qty.unwrap_or(10), 0).await?;

        Ok(response)
    }

    ////////

    /// # 7. [CASE] - 附近
    pub async fn logic_get_nearby_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<PoiListResponse> {
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        let poi_infos = PoiHomeService::find_city_gis_list(lat, lng, url.limit, url.offset).await?;

        let response =
            build_poi_list_response(poi_infos, url.uid, url.page.unwrap_or(1), url.qty.unwrap_or(10), 0).await?;

        Ok(response)
    }

    //////

    /// # 8. [CASE] - 精选
    pub async fn case_get_featured_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<PoiListResponse> {
        let poi_infos = PoiHomeService::find_featured_gis_list(url.limit, url.offset).await?;

        let response =
            build_poi_list_response(poi_infos, url.uid, url.page.unwrap_or(1), url.qty.unwrap_or(10), 0).await?;

        Ok(response)
    }

    /// # 9. [CASE] - 搜索
    pub async fn case_get_keyword_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<PoiListResponse> {
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        let poi_infos = PoiHomeService::search_gis_keyword_list(
            url.keyword,
            lat,
            lng,
            url.limit,
            url.offset,
        )
            .await?;

        let response =
            build_poi_list_response(poi_infos, url.uid, url.page.unwrap_or(1), url.qty.unwrap_or(10), 0).await?;

        Ok(response)
    }
}

//////// END