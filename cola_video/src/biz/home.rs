// cola_video/src/biz/home.rs  -- VIDEO - Biz - home
// 2026-06-05 00:20

////////

use anyhow::Result;
use cola_data::app::request::ApiUrlParamsQuery;
use crate::assembler::video::build_video_list_response;
use crate::model::vo::video::VideoListResponse;
use repo::video::service::video::VideoService; // 👈 整个文件，只依赖这一个唯一的 Service 服务通道

////////

/// # [LOGIC] - 1. 新的
pub async fn logic_get_new_list(
    query: ApiUrlParamsQuery,
) -> Result<VideoListResponse> {
    // 对接 VideoService
    let entities = VideoService::find_new_video_list(query.limit, query.offset).await?;

    let models = build_video_list_response(
        entities,
        query.uid,
        None, // 告别全局注入，用户上下文属性由装配层自行按需组装
        query.offset,
        query.limit,
        0,
    )
        .await?;

    Ok(models)
}

////////

/// # [LOGIC] - 2. 热门
pub async fn logic_get_hot_list(
    query: ApiUrlParamsQuery,
) -> Result<VideoListResponse> {
    // 对接 VideoService
    let entities = VideoService::find_hot_video_list(query.limit, query.offset).await?;

    let models = build_video_list_response(
        entities,
        query.uid,
        None,
        query.offset,
        query.limit,
        0,
    )
        .await?;

    Ok(models)
}

////////

/// # [LOGIC] - 3. 推荐
pub async fn logic_get_recommend_list(
    query: ApiUrlParamsQuery,
) -> Result<VideoListResponse> {
    // 对接 VideoService
    let entities = VideoService::find_recommend_video_list(query.limit, query.offset).await?;

    let models = build_video_list_response(
        entities,
        query.uid,
        None,
        query.offset,
        query.limit,
        0
    ).await?;

    Ok(models)
}

//////
/// # LOGIC 4. 同城
pub async fn logic_get_city_videos(
    query: ApiUrlParamsQuery,
    _city_id: i16,
) -> Result<VideoListResponse> {
    let lat = query.lat.unwrap_or(-4.4150144);
    let lng = query.lng.unwrap_or(114.016487);

    // 对接 VideoService
    let rows = VideoService::find_city_video_list(lat, lng, query.limit, query.offset).await?;

    // 从 Row 里提取出纯净的物理实体扔给装配器
    let entities = rows.into_iter().map(|r| r.entity).collect::<Vec<_>>();

    let models = build_video_list_response(
        entities,
        query.uid,
        None,
        query.offset,
        query.limit,
        0
    ).await?;

    Ok(models)
}

//////
/// # LOGIC 5. 附近
pub async fn logic_get_nearby_videos(
    query: ApiUrlParamsQuery,
) -> Result<VideoListResponse> {
    let lat = query.lat.unwrap_or(-4.4150144);
    let lng = query.lng.unwrap_or(114.016487);

    // 对接 VideoService
    let rows = VideoService::find_city_video_list(lat, lng, query.limit, query.offset).await?;
    let entities = rows.into_iter().map(|r| r.entity).collect::<Vec<_>>();

    let models = build_video_list_response(
        entities,
        query.uid,
        None,
        query.offset,
        query.limit,
        0
    ).await?;

    Ok(models)
}

//////

/// # [LOGIC] - 10. 精选
pub async fn logic_get_best_list(
    query: ApiUrlParamsQuery,
) -> Result<VideoListResponse> {
    // 完美的对接到 VideoService 的精选函数 👈 修正
    let entities = VideoService::find_featured_video_list(query.limit, query.offset).await?;

    let models = build_video_list_response(
        entities,
        query.uid,
        None,
        query.offset,
        query.limit,
        0
    ).await?;

    Ok(models)
}

/// # [LOGIC] - 11. 搜索
pub async fn logic_get_keyword_list(
    query: ApiUrlParamsQuery,
    keyword: String,
) -> Result<VideoListResponse> {
    let lat = query.lat.unwrap_or(-4.4150144);
    let lng = query.lng.unwrap_or(114.016487);

    // 完美的对接到 VideoService 的超级检索函数 👈 修正
    let rows = VideoService::search_video_keyword_list(
        &keyword,
        lat,
        lng,
        query.limit,
        query.offset,
    )
        .await?;

    let entities = rows.into_iter().map(|r| r.entity).collect::<Vec<_>>();

    let models = build_video_list_response(
        entities,
        query.uid,
        None,
        query.offset,
        query.limit,
        0
    ).await?;

    Ok(models)
}

//////// END