// cola_video/src/video/biz/me.rs  -- 我的视频业务逻辑
// 2026/4/24 14:35 by wx: cestbon10080

////////

use crate::user::port::user::UserPort;
use crate::video::assembler::video::build_video_list_response;
use crate::video::port::video::VideoPort;
use anyhow::Result;
use data::app::request::ApiUrlParamsQuery;
use data::video::model::video::{VideoListResponse, VideoVo};
use crate::ctx::AppContext;
////////

/// # [LOGIC] - 1. 我发布的视频
pub async fn logic_get_mine_list(
    query: ApiUrlParamsQuery,
    user_id: i64,
    ctx: &AppContext,
) -> Result<VideoListResponse> {

    let entities = ctx.video
        .video
        .find_me_publish_video_list(
            user_id,
            query.offset,
            query.limit,
        )
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "BIZ: 获取我的视频列表失败: {}",
                e
            )
        })?;

    let models = build_video_list_response(
        entities,
        Some(user_id),
        ctx.user.user.as_ref(),
        query.offset,
        query.limit,
        0,
    )
        .await?;

    Ok(models)
}

////////

/// # [LOGIC] - 2. 我看过的视频列表
pub async fn logic_get_my_visited_list(
    query: ApiUrlParamsQuery,
    user_id: i64,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> Result<VideoListResponse> {
    let video_ids = video_port
        .find_visited_video_ids(user_id, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取看过的视频IDs失败: {}", e))?;

    let entities = video_port
        .find_video_info_batch(video_ids, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取看过的视频列表失败: {}", e))?;

    let models = build_video_list_response(entities, Some(user_id), user_port, query.offset, query.limit, 0).await?;
    Ok(models)
}

////////

/// # [LOGIC] - 3. 我点赞的视频列表
pub async fn logic_get_my_liked_list(
    query: ApiUrlParamsQuery,
    user_id: i64,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> Result<VideoListResponse> {
    let video_ids = video_port
        .find_liked_video_ids(user_id, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取点赞的视频IDs失败: {}", e))?;

    let entities = video_port
        .find_video_info_batch(video_ids, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取点赞视频列表失败: {}", e))?;

    let models = build_video_list_response(entities, Some(user_id), user_port, query.offset, query.limit, 0).await?;
    Ok(models)
}

////////

/// # [LOGIC] - 4. 我收藏的视频
pub async fn logic_get_my_collected_list(
    query: ApiUrlParamsQuery,
    user_id: i64,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> Result<VideoListResponse> {
    let video_ids = video_port
        .find_collected_video_ids(user_id, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取收藏过的视频IDs失败: {}", e))?;

    let entities = video_port
        .find_video_info_batch(video_ids, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取收藏过视频列表失败: {}", e))?;

    let models = build_video_list_response(entities, Some(user_id), user_port, query.offset, query.limit, 0).await?;
    Ok(models)
}

////////

/// # [LOGIC] - 5. 我分享的视频列表
pub async fn logic_get_my_shared_list(
    query: ApiUrlParamsQuery,
    user_id: i64,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> Result<VideoListResponse> {
    let video_ids = video_port
        .find_shared_video_ids(user_id, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取分享过的视频IDs失败: {}", e))?;

    let entities = video_port
        .find_video_info_batch(video_ids, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取分享过视频列表失败: {}", e))?;

    let models = build_video_list_response(entities, Some(user_id), user_port, query.offset, query.limit, 0).await?;
    Ok(models)
}

////////

/// # [LOGIC] - 6. 我推荐的视频列表
pub async fn logic_get_my_recommend_list(
    query: ApiUrlParamsQuery,
    user_id: i64,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> Result<VideoListResponse> {
    let video_ids = video_port
        .find_recommend_video_ids(user_id, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取我推荐的视频IDs失败: {}", e))?;

    let entities = video_port
        .find_video_info_batch(video_ids, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取我推荐的视频列表失败: {}", e))?;

    let models = build_video_list_response(entities, Some(user_id), user_port, query.offset, query.limit, 0).await?;
    Ok(models)
}

////////

/// # [LOGIC] - 7. 我送上热门视频列表
pub async fn logic_get_my_hotlist_list(
    query: ApiUrlParamsQuery,
    user_id: i64,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> Result<VideoListResponse> {
    let video_ids = video_port
        .find_hotlist_video_ids(user_id, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取我送上过热门的视频IDs失败: {}", e))?;

    let entities = video_port
        .find_video_info_batch(video_ids, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取我送上热门的视频列表失败: {}", e))?;

    let models = build_video_list_response(entities, Some(user_id), user_port, query.offset, query.limit, 0).await?;
    Ok(models)
}

////////

/// # [LOGIC] - 8. 我购买的视频列表
pub async fn logic_get_my_buy_list(
    query: ApiUrlParamsQuery,
    user_id: i64,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> Result<VideoListResponse> {
    let video_ids = video_port
        .find_buy_video_ids(user_id, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取我的视频列表失败: {}", e))?;

    let entities = video_port
        .find_video_info_batch(video_ids, query.offset, query.limit)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 获取我的视频列表失败: {}", e))?;

    let models = build_video_list_response(entities, Some(user_id), user_port, query.offset, query.limit, 0).await?;
    Ok(models)
}

///////// END
