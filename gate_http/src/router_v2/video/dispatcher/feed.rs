// gate_http/src/router_v2/cola_video/dispatcher/feed.rs  -- VIDEO - dispatcher - feed
// 2026/8/1 08:22
// 2026/8/1 重构：纯路由转发器，按 action 路由到 handler

////////

use port::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::info::auth::AuthContext;
use cola_data::cola_auth::request::session::SessionContext;
use cola_video::api::feed::FeedApi;
use serde_json::Value;

////////

/// # [ACTION] - 动作码
pub mod action {
    pub const FEED_FOLLOWING: i16 = 2001; // 关注的人的
    pub const FEED_FRIEND: i16 = 2002; // 朋友的
    pub const FEED_RECOMMEND: i16 = 2003; // 为我推荐的
    pub const USER_PUBLISH: i16 = 2004; // TA发布的视频
    pub const USER_LIKED: i16 = 2005; // TA点赞的视频
    pub const USER_COLLECTED: i16 = 2006; // TA收藏的视频
    pub const USER_RECOMMEND: i16 = 2007; // TA推荐的视频
    pub const USER_NEARBY: i16 = 2008; // TA附近的视频
}

////////

/// # [DISPATCHER] - FEED 转发器（纯路由）
/// * `desc`: `只负责按 action 路由到 handler，不在此处验证会话`
/// * `session`: 由 gateway 验证后传入的可信上下文
pub async fn feed_dispatch(
    ctx: &AppContext,
    session: &SessionContext,
    req: &ApiGatewayRequest,
) -> AppData<Value> {
    let action = req.action.unwrap_or(2001);

    // 由可信 session 派生 AuthContext 传给 handler
    let auth = AuthContext::new(
        session.uid,
        session.access_token.clone(),
        String::new(),
        session.device_id.clone(),
    );

    let request = req.clone();

    ////////

    // 🚧 动作转发
    match action {
        // 关注的人的
        2001 => to_value(FeedApi::handler_feed_following(auth.clone(), request.clone(), ctx).await),

        // 朋友的
        2002 => to_value(FeedApi::handler_feed_friend(auth.clone(), request.clone(), ctx).await),

        // 为我推荐的
        2003 => to_value(FeedApi::handler_feed_recommend(auth.clone(), request.clone(), ctx).await),

        // TA发布的视频
        2004 => to_value(FeedApi::handler_user_publish(auth.clone(), request.clone(), ctx).await),

        // TA点赞的视频
        2005 => to_value(FeedApi::handler_user_liked(auth.clone(), request.clone(), ctx).await),

        // TA收藏的视频
        2006 => to_value(FeedApi::handler_user_collected(auth.clone(), request.clone(), ctx).await),

        // TA推荐的视频
        2007 => to_value(FeedApi::handler_user_recommend(auth.clone(), request.clone(), ctx).await),

        // TA附近的视频
        2008 => to_value(FeedApi::handler_user_nearby(auth.clone(), request.clone(), ctx).await),

        // more
        _ => AppData::err(
            4000,
            format!("[🚧 DISPATCH]: Unknown dispatch action: 💡[{}]", action),
            None,
        ),
    }
}

//////// END

/// 将具体 handler 的 AppData<T> 转换为转发器统一返回的 AppData<Value>
fn to_value(inner: AppData<impl serde::Serialize>) -> AppData<Value> {
    AppData {
        code: inner.code,
        message: inner.message,
        error: inner.error,
        duration: inner.duration,
        request_id: inner.request_id,
        at: inner.at,
        log_id: inner.log_id,
        data: inner.data.map(|d| serde_json::to_value(d).unwrap_or_default()),
    }
}