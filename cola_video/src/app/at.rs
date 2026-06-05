// cola_video/src/video/app/at.rs  -- 核心 - 短视频 - 应用 - 艾特我
// 2026/5/20 01:46 by wx: cestbon10080

////////

use crate::user::port::user::UserPort;
use crate::video::biz;
use crate::video::port::video::VideoPort;
use data::app::data::AppData;
use data::app::error;
use data::app::request::ApiUrlParamsQuery;
use data::auth::info::auth::AuthContext;
use data::video::model::video::VideoListResponse;
use crate::ctx::AppContext;
////////

/// # 1. [CASE] - 关注
pub async fn case_at_following(
    query: ApiUrlParamsQuery,
    auth: &AuthContext,
    ctx: &AppContext,
) -> AppData<VideoListResponse> {

    let user_id = match auth.user_id {
        Some(id) => id,

        None => {
            return AppData::err(
                error::UNAUTHORIZED,
                "未登录无法查看关注视频",
                None,
            )
        }
    };

    match biz::at::logic_get_following_list(
        query,
        user_id,
        ctx,
    )
        .await
    {
        Ok(resp) => AppData::ok(resp),

        Err(e) => {
            AppData::err(
                5002,
                e.to_string(),
                None,
            )
        }
    }
}

////////

/// # 2. [CASE] - 朋友
pub async fn case_at_friend(
    query: ApiUrlParamsQuery,
    user_port: &dyn UserPort,
    video_port: &dyn VideoPort,
) -> AppData<VideoListResponse> {
    match biz::at::logic_get_friend_list(query, user_port, video_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => AppData::err(5003, format!("朋友视频获取失败: {}", e), None),
    }
}

////////

/// # 3. [CASE] - 为我推荐
pub async fn case_at_recommend(
    query: ApiUrlParamsQuery,
    user_port: &dyn UserPort,
    video_port: &dyn VideoPort,
) -> AppData<VideoListResponse> {
    match biz::at::logic_get_recommend_list(query, user_port, video_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => AppData::err(5003, format!("推荐视频获取失败: {}", e), None),
    }
}

////////

/// # 4. [CASE] - 附近
pub async fn case_at_nearby(
    query: ApiUrlParamsQuery,
    user_port: &dyn UserPort,
    video_port: &dyn VideoPort,
) -> AppData<VideoListResponse> {
    match biz::at::logic_get_nearby_list(query, user_port, video_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => {
            tracing::error!("Nearby Error: {:?}", e);
            AppData::err(5001, "获取附近的视频失败", None)
        }
    }
}

//////// END
