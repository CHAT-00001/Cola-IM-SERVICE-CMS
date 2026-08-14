// cola_gis/src/api/mine.rs
// 可乐GIS - 接口层 - 我的
// 2026-07-07

////////

use crate::case::mine::MineCase;
use crate::case::feed::FeedCase;
use crate::model::vo::poi::PoiListResponse;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use port::app::ctx::AppContext;

////////

/// # [API HANDLER] - 我的 接口
pub struct MineApi;

impl MineApi {
    ////////

    /// # 1. [API HANDLER] - 我发布的兴趣点
    pub async fn handler_mine_publish(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<PoiListResponse> {
        let uid = auth.uid;

        match MineCase::case_mine_publish(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取我发布的兴趣点失败: {}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 我点赞的兴趣点
    pub async fn handler_mine_liked(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<PoiListResponse> {
        let uid = auth.uid;

        match MineCase::case_mine_liked(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取我点赞的兴趣点失败: {}", e), None),
        }
    }

    ////////

    /// # 3. [API HANDLER] - 附近的兴趣点
    pub async fn handler_mine_nearby(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<PoiListResponse> {
        let uid = auth.uid;

        match MineCase::case_mine_nearby(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取附近的兴趣点失败: {}", e), None),
        }
    }

    ////////

    /// # 4. [API HANDLER] - TA发布的兴趣点
    pub async fn handler_user_publish(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<PoiListResponse> {
        let uid = auth.uid;

        match FeedCase::case_feed_publish(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取TA发布的兴趣点失败: {}", e), None),
        }
    }

    ////////

    /// # 5. [API HANDLER] - TA点赞的兴趣点
    pub async fn handler_user_liked(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<PoiListResponse> {
        let uid = auth.uid;

        match FeedCase::case_feed_liked(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取TA点赞的兴趣点失败: {}", e), None),
        }
    }

    ////////

    /// # 6. [API HANDLER] - TA附近的兴趣点
    pub async fn handler_user_nearby(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<PoiListResponse> {
        let uid = auth.uid;

        match FeedCase::case_feed_nearby(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5005, format!("获取TA附近的兴趣点失败: {}", e), None),
        }
    }
}

//////// END