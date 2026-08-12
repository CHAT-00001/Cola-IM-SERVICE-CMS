// cola_gis/src/api/feed.rs
// 可乐GIS - 接口层 - feed流
// 2026-07-07

////////

use crate::case::feed::FeedCase;
use crate::model::vo::poi::PoiListResponse;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::info::auth::AuthContext;
use port::app::ctx::AppContext;

////////

/// # [API HANDLER] - 用户流 接口
pub struct FeedApi;

impl FeedApi {
    ////////

    /// # 1. [API HANDLER] - 关注的人的
    pub async fn handler_feed_following(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<PoiListResponse> {
        let uid = auth.uid;

        match FeedCase::case_feed_following(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取关注的人发布的兴趣点失败: {}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 用户发布的
    pub async fn handler_feed_publish(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<PoiListResponse> {
        let uid = auth.uid;

        match FeedCase::case_feed_publish(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取用户发布的兴趣点失败: {}", e), None),
        }
    }

    ////////

    /// # 3. [API HANDLER] - 用户点赞的
    pub async fn handler_feed_liked(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<PoiListResponse> {
        let uid = auth.uid;

        match FeedCase::case_feed_liked(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取用户点赞的兴趣点失败: {}", e), None),
        }
    }

    ////////

    /// # 4. [API HANDLER] - 附近的兴趣点
    pub async fn handler_feed_nearby(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<PoiListResponse> {
        let uid = auth.uid;

        match FeedCase::case_feed_nearby(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取附近的兴趣点失败: {}", e), None),
        }
    }
}