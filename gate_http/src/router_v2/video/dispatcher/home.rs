// gate_http/src/router_v2/video/dispatcher/home.rs  -- VIDEO - dispatcher - home
// 2026/7/18 16:14
// 2026/8/1 重构：纯路由转发器，不自带验证，返回通用 Value

////////

use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_data::auth::request::session::SessionContext;
use cola_video::api::home::HomeApi;
use serde_json::Value;

////////

/// # [ACTION] - 动作码
pub mod action {
    pub const HOME_NEW: i16 = 1001; // 最新
    pub const HOME_HOT: i16 = 1002; // 热门
    pub const HOME_CATE: i16 = 1003; // 分类
    pub const HOME_CHANNEL: i16 = 1004; // 频道
    pub const HOME_FEATURED: i16 = 1005; // 精选
    pub const HOME_SEARCH: i16 = 1006; // 搜索
}

////////

/// # [DISPATCHER] - HOME 转发器（纯路由）
/// * `desc`: `只负责按 action 路由到 handler，不在此处验证会话`
/// * `session`: 由 gateway 验证后传入的可信上下文
pub async fn home_dispatch(
    ctx: &AppContext,
    session: &SessionContext,
    req: &ApiGatewayRequest,
) -> AppData<Value> {
    let action = req.action.unwrap_or(1001);

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
        // 最新
        1001 => to_value(HomeApi::home_new(auth.clone(), request.clone(), ctx).await),

        // 热门
        1002 => to_value(HomeApi::home_hot(auth.clone(), request.clone(), ctx).await),

        // 分类
        1003 => to_value(HomeApi::home_category(auth.clone(), request.clone(), ctx).await),

        // 频道
        1004 => to_value(HomeApi::home_channel(auth.clone(), request.clone(), ctx).await),

        // 精选
        1005 => to_value(HomeApi::home_featured(auth.clone(), request.clone(), ctx).await),

        // 搜索
        1006 => to_value(HomeApi::home_search(auth.clone(), request.clone(), ctx).await),

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