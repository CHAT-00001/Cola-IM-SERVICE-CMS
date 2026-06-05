// usecase/src/live/app/me.rs  -- VIDEO - App - 我的
// 2026-04-16 08:00

////////

use crate::user::port::user::UserPort;
use crate::video::biz;
use crate::video::port::video::VideoPort;
use data::app::data::AppData;
use data::app::request::ApiUrlParamsQuery;
use data::auth::info::auth::AuthContext;
use data::video::model::video::VideoListResponse;
use crate::ctx::AppContext;
////////

/// # 1. [CASE] - 我发布的视频
pub async fn case_me_publish(
    query: ApiUrlParamsQuery,
    auth: &AuthContext,
    ctx: &AppContext,
) -> AppData<VideoListResponse> {

    let uid = match auth.user_id {
        Some(id) => id,

        None => {
            return AppData::err(
                4001,
                "请登录后查看我的视频",
                None,
            )
        }
    };

    match biz::me::logic_get_mine_list(
        query,
        uid,
        ctx,
    )
        .await
    {
        Ok(resp) => AppData::ok(resp),

        Err(e) => {
            tracing::error!(
                "GET_MINE_LIST_ERROR: {:?}",
                e
            );

            AppData::err(
                5004,
                "获取我的视频列表失败",
                Some(e.to_string()),
            )
        }
    }
}

////////

/// # 2. [CASE] - 我看过的视频
pub async fn case_me_visited(
    query: ApiUrlParamsQuery,
    auth: &AuthContext,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> AppData<VideoListResponse> {
    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(4001, "请登录后查看我看过的视频", None),
    };

    match biz::me::logic_get_my_visited_list(query, uid, video_port, user_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => {
            tracing::error!("GET_MINE_LIST_ERROR: {:?}", e);
            AppData::err(5004, "获取我看过的视频列表失败", Some(e.to_string()))
        }
    }
}

////////

/// # 3. [CASE] - 我点赞过的视频
pub async fn case_me_liked(
    query: ApiUrlParamsQuery,
    auth: &AuthContext,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> AppData<VideoListResponse> {
    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(4001, "请登录后查看点赞过的视频", None),
    };

    match biz::me::logic_get_my_liked_list(query, uid, video_port, user_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => {
            tracing::error!("GET_MINE_LIST_ERROR: {:?}", e);
            AppData::err(5004, "获取点赞过的视频列表失败", Some(e.to_string()))
        }
    }
}

////////

/// # 4. [CASE] - 我收藏过的视频
pub async fn case_me_collected(
    query: ApiUrlParamsQuery,
    auth: &AuthContext,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> AppData<VideoListResponse> {
    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(4001, "请登录后查看收藏过的视频", None),
    };

    match biz::me::logic_get_my_collected_list(query, uid, video_port, user_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => {
            tracing::error!("GET_MINE_LIST_ERROR: {:?}", e);
            AppData::err(5004, "获取收藏过的视频列表失败", Some(e.to_string()))
        }
    }
}

////////

/// # 5. [CASE] - 我推荐过的视频
pub async fn case_me_recommend(
    query: ApiUrlParamsQuery,
    auth: &AuthContext,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> AppData<VideoListResponse> {
    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(4001, "请登录后查看推荐过的视频", None),
    };

    match biz::me::logic_get_my_recommend_list(query, uid, video_port, user_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => {
            tracing::error!("GET_MINE_LIST_ERROR: {:?}", e);
            AppData::err(5004, "获取推荐过的视频列表失败", Some(e.to_string()))
        }
    }
}

////////

/// # 6. [CASE] - 我送上热门的视频
pub async fn case_me_hotlist(
    query: ApiUrlParamsQuery,
    auth: &AuthContext,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> AppData<VideoListResponse> {
    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(4001, "请登录后查看送上热门的视频", None),
    };

    match biz::me::logic_get_my_hotlist_list(query, uid, video_port, user_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => {
            tracing::error!("GET_MINE_LIST_ERROR: {:?}", e);
            AppData::err(5004, "获取送上热门的视频列表失败", Some(e.to_string()))
        }
    }
}

////////
