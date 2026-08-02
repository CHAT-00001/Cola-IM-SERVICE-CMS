// gate_http/src/router_v2/video/dispatcher/danmaku.rs  -- VIDEO - dispatcher - danmaku
// 2026/8/1 07:53
// 2026/8/1 重构：纯路由转发器，按 action 路由到 handler

////////

use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_data::auth::request::session::SessionContext;
use cola_data::video::command::danmaku::DanmakuCommand;
use cola_video::api::danmaku::DanmakuApi;
use serde_json::Value;

////////

/// # [ACTION] - 动作码
pub mod action {
    pub const DANMAKU_ADD: i16 = 4001; // 发布弹幕
    pub const DANMAKU_LIST: i16 = 4002; // 获取弹幕列表
}

////////

/// # [DISPATCHER] - DANMAKU 转发器（纯路由 + cmd 自适应映射）
/// * `desc`: `只负责按 action 路由到 handler；cmd 从 body.cmd 动态反序列化为对应命令结构体`
/// * `session`: 由 gateway 验证后传入的可信上下文
pub async fn danmaku_dispatch(
    ctx: &AppContext,
    session: &SessionContext,
    req: &ApiGatewayRequest,
) -> AppData<Value> {
    let action = req.action.unwrap_or(4001);
    let uid = session.uid;

    // 由可信 session 派生 AuthContext 传给 handler
    let auth = AuthContext::new(
        uid,
        session.access_token.clone(),
        String::new(),
        session.device_id.clone(),
    );

    // 💡 从透传的 body JSON 中提取 cmd 子对象（不同任务的结构不同）
    let cmd_value: Value = req
        .body
        .as_ref()
        .and_then(|b| b.get("cmd"))
        .cloned()
        .unwrap_or_else(|| Value::Object(Default::default()));

    ////////

    // 🚧 动作转发与动态反序列化 Command（自适应映射）
    match action {
        // 发布弹幕 (4001) → DanmakuCommand
        4001 => {
            let mut cmd: DanmakuCommand = match serde_json::from_value(cmd_value) {
                Ok(c) => c,
                Err(e) => return AppData::err(4000, format!("[🚧 DISPATCH]: 弹幕参数解析失败: {}", e), None),
            };
            cmd.user_id = uid; // 强制注入可信 uid
            to_value(DanmakuApi::handler_add_danmaku(auth, req.video_id, cmd).await)
        }

        // 获取弹幕列表 (4002)：ctx 需要可变引用，clone 一份后传入
        4002 => {
            let mut ctx_mut = ctx.clone();
            to_value(DanmakuApi::handler_get_danmaku(uid, req.video_id, 0, &mut ctx_mut).await)
        }

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