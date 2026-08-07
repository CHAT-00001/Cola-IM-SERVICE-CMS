// cola_user/src/api/user/feed.rs
// 可乐短用户 - api - 用户 - feed流
// 2026/5/20 02:04

////////

use crate::case::feed::FeedCase;
use crate::model::vo::video::VideoListResponse;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::auth::info::auth::AuthContext;

////////

/// # [API HANDLER] - 推荐流
/// * `desc`: `用户推荐流接口`
pub struct UserFeedApi;

// 构造函数
impl UserFeedApi {
    //
    ////////

    /// # 1. [API HANDLER] - 关注的人的
    pub async fn api_feed_following(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid.clone();

        // Call Case:
        match FeedCase::case_feed_following(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => AppData::err(5005, format!("获取关注的人发布的用户失败: {}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 朋友的
    pub async fn api_feed_friend(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid.clone();

        // Call Case:
        match FeedCase::case_feed_friend(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => AppData::err(5005, format!("获取朋友发布的用户失败: {}", e), None),
        }
    }

    ////////

    /// # 3. [API HANDLER] - 为我推荐的
    pub async fn api_feed_recommend(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid.clone();

        // Call Case:
        match FeedCase::case_feed_friend(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => AppData::err(5005, format!("获取关注的人发布的用户失败: {}", e), None),
        }
    }

    /// # 4. [API HANDLER] - TA发布的用户
    pub async fn api_user_publish(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid.clone();

        match FeedCase::case_feed_publish(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => AppData::err(5005, format!("获取TA发布的用户失败: {}", e), None),
        }
    }

    ////////

    /// # 5. [API HANDLER] - TA点赞的用户
    pub async fn api_user_liked(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid.clone();
        //
        // // 1. 检查会话状态
        // let auth_res = ensure_user_active(auth, session_port).await;
        // if auth_res.code != 0 {
        //     return AppData::err(auth_res.code, auth_res.message, None);
        // }

        // 2. 权限检查

        // 3. 调用用例执行
        match FeedCase::case_feed_liked(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取TA点赞的用户失败: {}", e), None),
        }
    }

    ////////

    /// # 6. [API HANDLER] - TA收藏的用户
    pub async fn api_user_collected(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid.clone();

        match FeedCase::case_feed_collected(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取TA收藏的用户失败: {}", e), None),
        }
    }

    ////////

    /// # 7. [API HANDLER] - TA推荐的用户
    pub async fn api_user_recommend(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid.clone();

        match FeedCase::case_user_recommend(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取TA推荐的用户失败: {}", e), None),
        }
    }

    ////////

    /// # 8. [API HANDLER] - TA附近的用户
    pub async fn api_user_nearby(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid.clone();

        match FeedCase::case_feed_nearby(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取TA附近的用户失败: {}", e), None),
        }
    }
}

//////// END
