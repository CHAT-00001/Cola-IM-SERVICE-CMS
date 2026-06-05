// cola_auth/src/app/session  -- AUTH - 应用层 - 业务逻辑
// 2026/06/05 05:10

////////

use crate::biz;
use crate::model::vo::session::SessionResponse;
use cola_data::app::data::AppData;
use cola_data::auth::command::login::AuthLoginCommand;
use validator::Validate;
use cola_data::app::api::ApiQuery;
////////

pub struct SessionCase;

impl SessionCase {

    ////////

    /// # [APP USE CASE] - 登录操作 (手机验证码)
    /// * `cmd`: 登录指令载荷 (由 AuthLogin 转换而来)
    /// * 机制：纯静态路由转发，彻底干掉 user_port 和 session_port 注入
    pub async fn case_login_by_phone(cmd: AuthLoginCommand) -> AppData<SessionResponse> {
        // 1. 简单校验
        if cmd.phone_no.is_empty() {
            return AppData::err(400, "手机号必填", None);
        }

        // 2. 🚀 剥离 Port，直接单向静态丢给本地的 Biz 逻辑
        match biz::session::logic_auth_login_by_phone(&cmd).await {
            Ok(res) => AppData::ok(res),
            Err(e) => AppData::err(500, &e.to_string(), None), // 错误即时阻断
        }
    }

    ////////

    /// # [APP USE CASE] - 退出登录
    /// * `uid`: 当前登录的用户 ID
    pub async fn case_logout(uid: i64) -> AppData<String> {
        // 1. 参数检查
        if uid <= 0 {
            return AppData::err(401, "非法用户ID", None);
        }

        // 2. 🚀 剥离 Port，直接静态调用本地 Biz 层逻辑（内部让 Token 失效）
        match biz::session::logic_auth_logout(uid).await {
            Ok(_) => AppData::ok("已安全退出".to_string()),
            Err(e) => AppData::err(500, &format!("退出失败: {}", e), None),
        }
    }

    ////////

    /// # [APP USE CASE] - 发送短信验证码
    /// * `phone`: 手机号
    pub async fn case_send_sms(phone: &String) -> AppData<String> {
        // 1. 基础校验
        if phone.is_empty() {
            return AppData::err(400, "手机号不能为空", None);
        }

        // 2. 🚀 剥离 Port，静态下钻到本地的验证码 Biz
        match crate::biz::sms::logic_send_sms_code(phone).await {
            Ok(_) => AppData::ok("验证码发送成功".to_string()),
            Err(e) => AppData::err(500, &e.to_string(), None),
        }
    }

    pub(crate) async fn logic_check_session_status(_p0: &i64) {
        todo!()
    }

    ////////

    /// # [CASE] - 刷新 Token
    /// * 返回: 最新的 Token
    pub async fn case_refresh_token(
        query: ApiQuery,
        auth_port: &dyn SessionPort,
    ) -> AppData<SessionResponse> {
        // 1. 明确获取 refresh_token 字段
        let r_token = match query.refresh_token.as_deref() {
            Some(t) if !t.is_empty() => t,
            _ => return AppData::err(error::PARAM_ERROR, "缺少刷新令牌", None),
        };

        // 2. 调用 Port 层进行刷新逻辑
        // 适配器内部会校验 r_token，如果有效，则生成新的 access_token 并返回最新的 Session
        match auth_port.refresh_session(r_token).await {
            Ok(new_session) => {
                // 3. 成功：返回新的会话信息
                AppData::ok(new_session).with_msg("令牌刷新成功")
            }
            Err(e) => {
                tracing::error!("REFRESH TOKEN ERROR: {:?}", e);
                AppData::err(error::UNAUTHORIZED, "刷新令牌已失效，请重新登录", None)
            }
        }
    }
}

//////// END
