// cola_auth/src/api/session.rs  -- AUTH - 接口层 - 会话
// 2026/06/05 05:10

////////

use crate::case::session::SessionCase;
use crate::model::vo::session::SessionResponse;
use cola_data::app::api::ApiQuery;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::auth::command::phone::PhoneLoginCommand;
use tracing::log;
use validator::Validate;
use cola_data::auth::info::session::AccessTokenInfo;
////////

/// # [API] - 会话接口
pub struct SessionApi;

impl SessionApi {
    ////////

    /// # 1. [APP USE CASE] - 退出登录
    /// * action: 4001
    /// * `uid`: 当前登录的用户 ID
    pub async fn handler_logout(
        uid: i64,         // 操作者 ID
        session_id: &str, // 会话 ID
        device_id: &str,  //  设备 ID
    ) -> AppData<String> {
        // 1. 参数检查
        if uid <= 0 {
            return AppData::err(401, "非法用户ID", None);
        }

        // 2. 🚀 剥离 Port，直接静态调用本地 Biz 层逻辑（内部让 Token 失效）
        match SessionCase::case_logout(uid, session_id, device_id).await {
            Ok(_) => AppData::ok("已安全退出".to_string()),
            Err(e) => AppData::err(500, &format!("退出失败: {}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 刷新 Token
    /// * action: 7001
    /// * 返回: 最新的 Token
    pub async fn handler_refresh_token(query: ApiQuery) -> AppData<AccessTokenInfo> {
        // 1. 错误分支：校验入参
        let r_token = match query.refresh_token.as_deref() {
            Some(t) if !t.is_empty() => t,
            _ => return AppData::err(error::PARAM_ERROR, "缺少刷新令牌", None),
        };

        // 2. 逻辑处理分支
        match SessionCase::case_refresh(r_token).await {
            Ok(info) => AppData::ok(info),
            Err(e) => {
                tracing::error!("REFRESH TOKEN ERROR: {:?}", e);
                AppData::err(error::UNAUTHORIZED, "刷新失败，请重新登录", None)
            }
        }
    }
}

//////// END
