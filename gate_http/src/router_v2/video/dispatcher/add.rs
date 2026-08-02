// gate_http/src/router_v2/new/dispatcher/add.rs
// HTTP 网关 - v2 - VIDEO - dispatcher - add
// 2026/8/1 07:54
// 2026/8/1 重构：纯路由转发器 + 从 body.cmd 自适应动态映射命令结构体

////////

use actix_web::web::service;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::request::session::SessionContext;
use cola_data::video::command::video::edit::VideoUpdateCommand;
use cola_data::video::command::video::new::VideoNewCommand;
use cola_data::video::command::video::permission::VideoUpdatePermissionCommand;
use cola_video::api::add::AddApi;
use serde_json::Value;
use tracing::log::info;
////////

/// # [ACTION] - 动作码
pub mod action {
    pub const ADD_PUBLISH: i16 = 5001; // 发布视频
    pub const ADD_EDIT: i16 = 5002; // 编辑视频
    pub const ADD_STATE: i16 = 5003; // 修改状态(上架/下架/删除/隐藏..)
    pub const ADD_PERM: i16 = 5004; // 修改权限(浏览/评论/弹幕/分享/下载/..)
    pub const ADD_LBS: i16 = 5005; // 添加LBS服务
}

////////

/// # [DISPATCHER] - 转发器（纯路由 + cmd 自适应映射）
/// * `desc`: `只负责按 action 路由到 handler；cmd 从 body.cmd 动态反序列化为对应命令结构体`
/// * `session`: 由 gateway 验证后传入的可信上下文
pub async fn add_dispatch(
    _ctx: &AppContext,
    session: &SessionContext,
    req: &ApiGatewayRequest,
) -> AppData<Value> {
    let action = req.action.unwrap_or(5001);
    let uid = session.uid;

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
        // 发布视频 (5001) → VideoNewCommand
        5001 => {
            info!("service_name: [👤 ADD] - action: [💬 PUBLISH - 发布视频]");
            let mut cmd: VideoNewCommand = match serde_json::from_value(cmd_value) {
                Ok(c) => c,
                Err(e) => {
                    return AppData::err(
                        4000,
                        format!("[🚧 DISPATCH]: 发布参数解析失败: {}", e),
                        None,
                    );
                }
            };
            cmd.uid = uid; // 强制注入可信 uid
            to_value(AddApi::add_publish(uid, cmd).await)
        }

        // 编辑视频 (5002) → VideoUpdateCommand
        5002 => {
            let cmd: VideoUpdateCommand = match serde_json::from_value(cmd_value) {
                Ok(c) => c,
                Err(e) => {
                    return AppData::err(
                        4000,
                        format!("[🚧 DISPATCH]: 编辑参数解析失败: {}", e),
                        None,
                    );
                }
            };
            to_value(AddApi::add_edit(uid, cmd).await)
        }

        // 修改状态 (5003) → 复用 VideoNewCommand（handler 签名一致）
        5003 => {
            let mut cmd: VideoNewCommand = match serde_json::from_value(cmd_value) {
                Ok(c) => c,
                Err(e) => {
                    return AppData::err(
                        4000,
                        format!("[🚧 DISPATCH]: 状态参数解析失败: {}", e),
                        None,
                    );
                }
            };
            cmd.uid = uid;
            to_value(AddApi::add_status(uid, cmd).await)
        }

        // 修改权限 (5004) → VideoUpdatePermissionCommand
        5004 => {
            let cmd: VideoUpdatePermissionCommand = match serde_json::from_value(cmd_value) {
                Ok(c) => c,
                Err(e) => {
                    return AppData::err(
                        4000,
                        format!("[🚧 DISPATCH]: 权限参数解析失败: {}", e),
                        None,
                    );
                }
            };
            to_value(AddApi::add_permission(uid, cmd).await)
        }

        // 添加 lbs (5005) → 复用 VideoNewCommand（目前 handler 签名一致）
        5005 => {
            let mut cmd: VideoNewCommand = match serde_json::from_value(cmd_value) {
                Ok(c) => c,
                Err(e) => {
                    return AppData::err(
                        4000,
                        format!("[🚧 DISPATCH]: LBS参数解析失败: {}", e),
                        None,
                    );
                }
            };
            cmd.uid = uid;
            to_value(AddApi::add_lbs(uid, cmd).await)
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
        data: inner
            .data
            .map(|d| serde_json::to_value(d).unwrap_or_default()),
    }
}
