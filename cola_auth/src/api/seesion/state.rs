// auth/src/api/session/state.rs
// core - AUTH - api - session - 状态
// 2026/04/13 10:15

////////

use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::auth::info::session::AccessTokenInfo;
use cola_data::auth::request::session::{AuthSessionRequest, SessionContext};
use port::auth::AuthServicePorts;
use crate::case::session::state::SessionStateCase;
////////

/// # [STATE HANDLER] - 会话 状态
pub struct SessionStateApi;

// 构造实现
impl SessionStateApi {
    //

    ////////

    /// # 1. [API] - verify_session
    /// 从验证中心（Auth）校验 Token，并生成可信 SessionContext
    pub async fn verify_session(
        auth: &AuthSessionRequest,
        auth_port: &AuthServicePorts,
    ) -> AppData<SessionContext> {
        // 1. 游客访问
        if !auth.has_token() {
            return AppData::ok(SessionContext::anonymous());
        }

        // 2. 获取 Token
        let token = auth.access_token.as_ref().unwrap();

        // 3. 调用认证中心（SessionPort adapter）
        let session_opt = match auth_port.session.get_session(token).await {
            Ok(opt) => opt,
            Err(e) => {
                tracing::error!("AUTH_PORT ERROR: {:?}", e);
                return AppData::err(error::INTERNAL_ERROR, "验证服务暂时不可用", None);
            }
        };

        // 4. Token 是否有效
        let session = match session_opt {
            Some(s) => s,
            None => {
                return AppData::err(error::UNAUTHORIZED, "登录状态已过期", None);
            }
        };

        // 5. 用户状态
        if session.user_info.status == 0 {
            return AppData::err(error::FORBIDDEN, "当前账号已被冻结，请联系客服", None);
        }

        // 6. 构建可信 SessionContext
        let ctx = SessionContext::from_token(
            session.user_info.uid,   // ← 建议 SessionResponse 直接提供 uid:i64
            session.user_info.roles, // ← Vec<String>
            auth.device_id.clone().unwrap_or_default(),
            token.clone(),
        );

        AppData::ok(ctx)
    }

    ////////

    /// # 2. [API] - verify_login
    /// 校验必须登录，游客将返回未登录
    pub async fn verify_login(
        auth: &AuthSessionRequest,
        auth_port: &AuthServicePorts,
    ) -> AppData<SessionContext> {
        let result = Self::verify_session(auth, auth_port).await;

        match result {
            AppData {
                data: Some(ctx), ..
            } if !ctx.is_anonymous => AppData::ok(ctx),

            AppData { data: Some(_), .. } => AppData::err(error::UNAUTHORIZED, "请先登录", None),

            err => err.rebind(),
        }
    }

    ////////

    /// # 3. [API] - verify_uid
    /// 获取当前操作者 UID
    pub async fn verify_uid(
        auth: &AuthSessionRequest,
        auth_port: &AuthServicePorts,
    ) -> Result<i64, AppData<i64>> {
        match Self::verify_login(auth, auth_port).await {
            AppData {
                data: Some(ctx), ..
            } => Ok(ctx.uid),

            err => Err(err.rebind()),
        }
    }

    ////////

    /// # 4. [API] - Refresh Token 刷新token
    /// * `desc`: `刷新access_token`
    pub async fn refresh_token(
        auth: &AuthSessionRequest,
    ) -> AppData<AccessTokenInfo> {
        let refresh_token = match auth.refresh_token.as_deref() {
            Some(token) if !token.trim().is_empty() => token,
            _ => return AppData::err(error::PARAM_ERROR, "缺少刷新令牌", None),
        };

        match SessionStateCase::case_refresh(refresh_token).await {
            Ok(info) => AppData::ok(info),
            Err(err) => {
                tracing::error!("[🤐 AUTH API] - ❌️ 刷新 access_token 失败: {:?}", err);
                AppData::err(error::UNAUTHORIZED, "刷新失败，请重新登录", None)
            }
        }
    }
}

//////// END
