// cola_video/src/video/biz/user  -- VIDEO - Biz - TA 他的
// 2026/5/23 00:01 by wx: cestbon10080

////////

use crate::user::port::user::UserPort;
use crate::video::assembler::video::build_video_list_response;
use crate::video::port::ta::VideoTaPort;
use crate::video::port::video::VideoPort;
use data::app::request::ApiUrlParamsQuery;
use data::video::model::video::VideoListResponse;
use crate::ctx::AppContext;

////////

/// # [LOGIC] - 1. 用户发布的视频
pub async fn logic_get_user_publish(
    query: ApiUrlParamsQuery,
    target_user_id: i64,
    ctx: &AppContext,
) -> anyhow::Result<VideoListResponse> {

    let entities = ctx.video
        .video
        .find_user_video_list(
            target_user_id,
            query.offset,
            query.limit,
        )
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "BIZ: 获取用户 {} 视频列表失败: {}",
                target_user_id,
                e
            )
        })?;

    let resp = build_video_list_response(
        entities,
        Some(target_user_id),
        ctx.user.user.as_ref(),
        query.offset,
        query.limit,
        0,
    )
        .await?;

    Ok(resp)
}

//////

/// # [LOGIC] - 2. 他点赞的视频
pub async fn logic_get_user_liked(
    query: ApiUrlParamsQuery,
    taget_user_id: i64,
    ta_port: &dyn VideoTaPort,
    _user_port: &dyn UserPort,
) -> anyhow::Result<VideoListResponse> {
    let entities = ta_port
        .find_ta_likes_list(taget_user_id, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 点赞的列表失败: {}", taget_user_id, e))?;

    let resp = build_video_list_response(entities, Some(taget_user_id), _user_port, query.offset, query.limit, 0).await?;
    Ok(resp)
}

//////

/// # [LOGIC] - 3. 他收藏的视频
pub async fn logic_get_user_collected(
    query: ApiUrlParamsQuery,
    taget_user_id: i64,
    ta_port: &dyn VideoTaPort,
    _user_port: &dyn UserPort,
) -> anyhow::Result<VideoListResponse> {
    let entities = ta_port
        .find_ta_collect_list(taget_user_id, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 收藏的列表失败: {}", taget_user_id, e))?;

    let resp = build_video_list_response(entities, Some(taget_user_id), _user_port, query.offset, query.limit, 0).await?;
    Ok(resp)
}

//////

/// # [LOGIC] - 4. 他推荐的视频
pub async fn logic_get_user_recommend(
    query: ApiUrlParamsQuery,
    taget_user_id: i64,
    ta_port: &dyn VideoTaPort,
    _user_port: &dyn UserPort,
) -> anyhow::Result<VideoListResponse> {
    let entities = ta_port
        .find_ta_collect_list(taget_user_id, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 推荐的列表失败: {}", taget_user_id, e))?;

    let resp = build_video_list_response(entities, Some(taget_user_id), _user_port, query.offset, query.limit, 0).await?;
    Ok(resp)
}

//////

/// # [LOGIC] - 5. 他附近的视频
pub async fn logic_get_user_nearby(
    query: ApiUrlParamsQuery,
    taget_user_id: i64,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> anyhow::Result<VideoListResponse> {
    let lat = query.lat.unwrap_or(0.0);
    let lng = query.lng.unwrap_or(0.0);

    let entities = video_port
        .find_nearby_video_list(lat, lng, 1000.0, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取用户 {} 附近视频失败: {}", taget_user_id, e))?;

    let resp = build_video_list_response(entities, Some(taget_user_id), user_port, query.offset, query.limit, 0).await?;
    Ok(resp)
}

/////// END
