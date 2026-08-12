// auth/src/api/session.rs  -- AUTH - 接口层 - 会话
// 2026/06/05 05:10

//////

use crate::case::session::SessionCase;
use cola_data::cola_auth::vo::session::SignResponse;
use cola_data::app::api::ApiQuery;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_auth::command::phone::PhoneLoginCommand;
use cola_data::cola_auth::info::session::AccessTokenInfo;
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
    //
    // /// # [API] - 验证访问令牌
    // ///
    // /// 供 UGC 等业务服务调用，验证 Token 有效性并返回用户信息
    // pub async fn handler_verify_token(
    //     access_token: &str,
    // ) -> AppData<VerifyTokenResult> {
    //     // 1. 参数校验
    //     if access_token.is_empty() {
    //         return AppData::err(error::PARAM_ERROR, "缺少访问令牌", None);
    //     }
    //
    //     // 2. 调用 Case 层验证
    //     match SessionCase::case_verify_access_token(access_token).await {
    //         Ok(user_info) => AppData::ok(user_info),
    //         Err(e) => {
    //             tracing::warn!("TOKEN VERIFY FAILED: {:?}", e);
    //             AppData::err(error::UNAUTHORIZED, "令牌无效或已过期", None)
    //         }
    //     }
    // }
}

//////// END
