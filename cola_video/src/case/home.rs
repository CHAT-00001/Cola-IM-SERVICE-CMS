<<<<<<< HEAD
// cola_video/src/case/video/home.rs  -- VIDEO - 用例层 - 视频内容 - home用例
=======
// cola_video/src/case/home.rs  -- VIDEO - 用例层 - home用例
>>>>>>> 4c64c9b4c5a10e3d2745c48010789b9c5b2eba9a
// 2026-06-11 08:10

////////

use crate::assembler::video::build_video_list_response_with_cdn;
use crate::case::storage::resolve_video_cdn_domain;
use anyhow::Result;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_video::info::video::VideoListResponse;
use port::app::ctx::AppContext;
use service::cola_video::video::list::VideoListService;

////////

/// # [HOME CASE] - 视频主页用例编排
/// * `desc`: `COLA VIDEO - User Case Ep`
pub struct HomeCase;

impl HomeCase {
<<<<<<< HEAD
    // 🚧 - CASE - 用例编排
=======
    /// 💡 - 所有CASE
>>>>>>> 4c64c9b4c5a10e3d2745c48010789b9c5b2eba9a

    ////////

    /// # 2. [CASE] - 新的
    pub async fn case_get_new_list(
        uid: i64, // 操作者 ID
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        // 🚧 - CASE - 开始编排

        ////////

        // 💡 - CDN - 域名
        let cdn_domain = resolve_video_cdn_domain(ctx, "short-video").await?;

        ////////

        // 💡 - INFOS - 视频信息
        let video_infos = VideoListService::find_new_video_list(url.limit, url.offset).await?;

        ////////

        // 💡 - RESPONSE - 响应组装
        let response = build_video_list_response_with_cdn(
            video_infos,
            Some(uid),
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
            &cdn_domain,
        )
        .await?;

        ////////

        // ✅️ - Ok
        Ok(response)
    }

    ////////

    /// # 3. [CASE] - 热门
    pub async fn case_get_hot_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        // 🚧 - CASE - 开始编排

        ////////

        // 💡 - CDN - 域名
        let cdn_domain = resolve_video_cdn_domain(ctx, "short-video").await?;

        ////////

        // 💡 - INFOS - 视频信息
        let video_infos = VideoListService::find_hot_video_list(url.limit, url.offset).await?;

        ////////

        // 💡 - RESPONSE - 响应组装
        let response = build_video_list_response_with_cdn(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
            &cdn_domain,
        )
        .await?;

        ////////

        // ✅️ - Ok
        Ok(response)
    }

    ////////

    /// # 4. [CASE] - 推荐
    pub async fn case_get_recommend_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        // 🚧 - 开始编排

        ////////

        // 💡 - CDN - 域名
        let cdn_domain = resolve_video_cdn_domain(ctx, "short-video").await?;

        ////////

        // 💡 - INFOS - 视频信息
        let video_infos =
            VideoListService::find_recommend_video_list(url.limit, url.offset).await?;

        ////////

        // 💡 - RESPONSE - 响应组装
        let response = build_video_list_response_with_cdn(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
            &cdn_domain,
        )
        .await?;

        ////////

        // ✅️ Ok
        Ok(response)
    }

    ////////

    /// # 5. [CASE] - 同城
    pub async fn case_get_city_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        // 🚧 - 开始编排

        ////////

        // 💡 - CDN - 域名
        let cdn_domain = resolve_video_cdn_domain(ctx, "short-video").await?;

        // 💡 - PARMAES - 参数
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        ////////

        // 💡 - INFOS - 视频信息
        let video_infos =
            VideoListService::find_city_video_list(lat, lng, url.limit, url.offset).await?;

        ////////

        // 💡 - RESPONSE - 响应组装
        let response = build_video_list_response_with_cdn(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
            &cdn_domain,
        )
        .await?;

        ////////

        // ✅️ - Ok
        Ok(response)
    }

    ////////

    /// # 6. [CASE] - 分类
    pub async fn case_get_category_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        //  🚧 - 开始编排

        ////////

        // 💡 - CDN - 域名
        let cdn_domain = resolve_video_cdn_domain(ctx, "short-video").await?;
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        // 🌟 已修正：直接一步到位拿到 video_infos
        let video_infos =
            VideoListService::find_city_video_list(lat, lng, url.limit, url.offset).await?;

        let response = build_video_list_response_with_cdn(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
            &cdn_domain,
        )
        .await?;

        Ok(response)
    }

    ////////

    /// # 7. [CASE] - 附近
    pub async fn logic_get_nearby_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        // 🚧 - 开始编排

        ////////

        // 💡 - CDN - 域名
        let cdn_domain = resolve_video_cdn_domain(ctx, "short-video").await?;
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        ////////

        // 💡 - INFOS - 视频信息
        let video_infos =
            VideoListService::find_city_video_list(lat, lng, url.limit, url.offset).await?;

        ////////

        // 💡 - RESPONSE - 响应组装
        let response = build_video_list_response_with_cdn(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
            &cdn_domain,
        )
        .await?;

        ////////

        // ✅️ - Ok
        Ok(response)
    }

    ////////

    /// # 8. [CASE] - 精选
    pub async fn case_get_featured_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        // 🚧 - CASE - 开始编排

        ////////

        // 💡 - CDN - 域名
        let cdn_domain = resolve_video_cdn_domain(ctx, "short-video").await?;

        ////////

        // 💡 - INFOS - CTX
        let video_infos = VideoListService::find_featured_video_list(url.limit, url.offset).await?;

        ////////

        // 💡 - RESPONSE
        let response = build_video_list_response_with_cdn(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
            &cdn_domain,
        )
        .await?;

        // ✅️ - Ok
        Ok(response)
    }

    ////////

    /// # 9. [CASE] - 搜索
    pub async fn case_get_keyword_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoListResponse> {
        // 🚧 - CASE - 开始编排

        ////////

        // 💡 - CDN - CTX
        let cdn_domain = resolve_video_cdn_domain(ctx, "short-video").await?;
        let lat = url.lat.unwrap_or(-4.4150144);
        let lng = url.lng.unwrap_or(114.016487);

        // 💡 - INFOS - CTX
        let video_infos = VideoListService::search_video_keyword_list(
            url.keyword,
            lat,
            lng,
            url.limit,
            url.offset,
        )
        .await?;

        ////////

        // 💡 - RESPONSE - 响应组装
        let response = build_video_list_response_with_cdn(
            video_infos,
            url.uid,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10),
            0,
            &cdn_domain,
        )
        .await?;

        ////////

        // ✅ - OK
        Ok(response)
    }
}

//////// END
