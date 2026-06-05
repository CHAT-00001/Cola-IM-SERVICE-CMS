// cola_video/src/video/app/user  -- VIDEO - 应用层 - TA的
// 2026/5/20 02:04 by wx: cestbon10080

//////

use crate::auth::app::active::ensure_user_active;
use crate::auth::port::session::SessionPort;
use crate::user::port::user::UserPort;
use crate::video::biz;
use crate::video::port::ta::VideoTaPort;
use crate::video::port::video::VideoPort;
use data::app::data::AppData;
use data::app::request::ApiUrlParamsQuery;
use data::auth::info::auth::AuthContext;
use data::video::model::video::VideoListResponse;
use crate::ctx::AppContext;
//////

/// # 1. [CASE] - TA发布的视频
pub async fn case_user_publish(
    query: ApiUrlParamsQuery,
    target_user_id: i64,
    auth: &AuthContext,
    ctx: &AppContext,
) -> AppData<VideoListResponse> {

    let auth_res = ensure_user_active(
        auth,
        ctx.auth.session.as_ref(),
    )
        .await;

    if auth_res.code != 0 {
        return AppData::err(
            auth_res.code,
            auth_res.message,
            None,
        );
    }

    match biz::ta::logic_get_user_publish(
        query,
        target_user_id,
        ctx,
    )
        .await
    {
        Ok(resp) => AppData::ok(resp),

        Err(e) => {
            AppData::err(
                5005,
                format!(
                    "获取TA发布的视频失败: {}",
                    e
                ),
                None,
            )
        }
    }
}

//////

/// # 2. [CASE] - TA点赞的视频
pub async fn case_ta_liked(
    query: ApiUrlParamsQuery,
    target_user_id: i64,
    auth: &AuthContext,
    session_port: &dyn SessionPort,
    ta_port: &dyn VideoTaPort,
    user_port: &dyn UserPort,
) -> AppData<VideoListResponse> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    match biz::ta::logic_get_user_liked(query, target_user_id, ta_port, user_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => AppData::err(5005, format!("获取TA点赞的视频失败: {}", e), None),
    }
}

//////

/// # 3. [CASE] - TA收藏的视频
pub async fn case_ta_collected(
    query: ApiUrlParamsQuery,
    target_user_id: i64,
    auth: &AuthContext,
    session_port: &dyn SessionPort,
    ta_port: &dyn VideoTaPort,
    user_port: &dyn UserPort,
) -> AppData<VideoListResponse> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    match biz::ta::logic_get_user_collected(query, target_user_id, ta_port, user_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => AppData::err(5005, format!("获取TA收藏的视频失败: {}", e), None),
    }
}

//////

/// # 4. [CASE] - TA推荐的视频
pub async fn case_ta_recommend(
    query: ApiUrlParamsQuery,
    target_user_id: i64,
    auth: &AuthContext,
    session_port: &dyn SessionPort,
    ta_port: &dyn VideoTaPort,
    user_port: &dyn UserPort,
) -> AppData<VideoListResponse> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    match biz::ta::logic_get_user_recommend(query, target_user_id, ta_port, user_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => AppData::err(5005, format!("获取TA推荐的视频失败: {}", e), None),
    }
}

//////

/// # 5. [CASE] - TA附近的视频
pub async fn case_ta_nearby(
    query: ApiUrlParamsQuery,
    target_user_id: i64,
    auth: &AuthContext,
    session_port: &dyn SessionPort,
    video_port: &dyn VideoPort,
    user_port: &dyn UserPort,
) -> AppData<VideoListResponse> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    match biz::ta::logic_get_user_nearby(query, target_user_id, video_port, user_port).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => AppData::err(5005, format!("获取TA附近的视频失败: {}", e), None),
    }
}

//////// END
