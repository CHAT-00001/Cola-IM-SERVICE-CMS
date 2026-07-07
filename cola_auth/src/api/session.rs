// cola_auth/src/api/session.rs  -- AUTH - 接口层 - 会话
// 2026/06/05 05:10

//////

use crate::case::session::SessionCase;
use crate::model::vo::session::SignResponse;
use cola_data::app::api::ApiQuery;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::auth::command::phone::PhoneLoginCommand;
use cola_data::auth::info::session::AccessTokenInfo;
use tracing::log;
use validator::Validate;

//////

/// # [API] - 会话 接口
pub struct SessionApi;

impl SessionApi {
    ////////

    /// # 1. [APP USE CASE] - 退出登录
    pub async fn handler_session_sign_out(
        uid: i64,
        session_id: &str,
        device_id: &str,
    ) -> AppData<String> {
        if uid <= 0 {
            return AppData::err(401, "非法用户ID", None);
        }

        match SessionCase::case_logout(uid, session_id, device_id).await {
            Ok(_) => AppData::ok("已安全退出".to_string()),
            Err(e) => AppData::err(500, &format!("退出失败: {}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 刷新 Token
    pub async fn handler_refresh_token(query: ApiQuery) -> AppData<AccessTokenInfo> {
        let r_token = match query.refresh_token.as_deref() {
            Some(t) if !t.is_empty() => t,
            _ => return AppData::err(error::PARAM_ERROR, "缺少刷新令牌", None),
        };

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
