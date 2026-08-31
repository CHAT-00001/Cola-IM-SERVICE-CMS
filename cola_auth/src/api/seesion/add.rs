// auth/src/api/session/add.rs  -- 可乐验证中心 - 接口层 - 登录
// 2026/6/9 07:54

////////

use crate::case::login::LoginCase;
use crate::case::session::add::AuthAddCase;
use cola_data::app::api::ApiQuery;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::auth::command::email::EmailLoginCommand;
use cola_data::auth::command::phone::PhoneLoginCommand;
use cola_data::auth::vo::session::SignResponse;
use port::app::ctx::AppContext;
use tracing::log;
use validator::Validate;

////////

/// # [ADD API] - 登录 接口
pub struct SessionAddApi;

impl SessionAddApi {
    //

    ////////

    /// # 1. [API HANDLER] - 手机短信验证码登录
    /// * `action`: 2001
    /// * `desc`: 手机 + 短信
    pub async fn handler_sign_in_by_phone(
        cmd: PhoneLoginCommand,
        ctx: &AppContext,
    ) -> AppData<SignResponse> {
        log::info!("[API]: 收到 📱 手机号登录原始命令数据: {:?}", cmd);

        // 1. 校验
        if let Err(e) = cmd.validate_params() {
            return AppData::err(4000, &format!("参数校验失败: {}", e), None);
        }

        // 2. 调度业务逻辑
        match AuthAddCase::case_sign_in_by_phone(&cmd, ctx).await {
            Ok(res) => AppData::ok(res),
            Err(e) => {
                log::error!("登录失败: {:?}", e);
                AppData::err(5000, "登录失败，请稍后再试", None)
            }
        }
    }

    ////////

    /// # 2. [API HANDLER] - 邮箱验证码登录
    /// * `action`: 2002
    /// * `desc`: email + code
    pub async fn handler_sign_in_by_email(cmd: EmailLoginCommand) -> AppData<SignResponse> {
        log::info!("[API]: 收到 📮 邮箱验证码登录原始命令数据: {:?}", cmd);

        if let Err(e) = cmd.validate_params() {
            return AppData::err(4000, &format!("参数校验失败: {}", e), None);
        }

        match AuthAddCase::case_sign_in_by_email(&cmd).await {
            Ok(res) => AppData::ok(res),
            Err(e) => {
                log::error!("登录失败: {:?}", e);
                AppData::err(5000, "登录失败，请稍后再试", None)
            }
        }
    }

    ////////

    /// # 3. [API HANDLER] - 账号密码登录
    /// * `action`: 2003
    pub async fn handler_sign_in_by_pwd(cmd: PhoneLoginCommand) -> AppData<SignResponse> {
        if let Err(e) = cmd.validate_params() {
            return AppData::err(400, &format!("参数校验失败: {}", e), None);
        }

        match LoginCase::case_phone(&cmd).await {
            Ok(res) => AppData::ok(res),
            Err(e) => {
                log::error!("登录失败: {:?}", e);
                AppData::err(500, "登录失败，请稍后再试", None)
            }
        }
    }

    ////////

    /// # 4. [API HANDLER] - 谷歌登录
    /// * `desc`: 2004
    pub async fn handler_sign_in_by_google(cmd: PhoneLoginCommand) -> AppData<SignResponse> {
        if let Err(e) = cmd.validate_params() {
            return AppData::err(400, &format!("参数校验失败: {}", e), None);
        }

        match LoginCase::case_phone(&cmd).await {
            Ok(res) => AppData::ok(res),
            Err(e) => {
                log::error!("登录失败: {:?}", e);
                AppData::err(500, "登录失败，请稍后再试", None)
            }
        }
    }

    ////////

    /// # 5. [API HANDLER] - 苹果登录
    /// * `action`: 2005
    pub async fn handler_sign_in_by_apple(cmd: PhoneLoginCommand) -> AppData<SignResponse> {
        if let Err(e) = cmd.validate_params() {
            return AppData::err(400, &format!("参数校验失败: {}", e), None);
        }

        match LoginCase::case_phone(&cmd).await {
            Ok(res) => AppData::ok(res),
            Err(e) => {
                log::error!("登录失败: {:?}", e);
                AppData::err(500, "登录失败，请稍后再试", None)
            }
        }
    }

    ////////

    /// # 6. [API HANDLER] - 微信登录
    /// * `action`: 2006
    pub async fn handler_sign_in_by_wechat(cmd: PhoneLoginCommand) -> AppData<SignResponse> {
        if let Err(e) = cmd.validate_params() {
            return AppData::err(400, &format!("参数校验失败: {}", e), None);
        }

        match LoginCase::case_phone(&cmd).await {
            Ok(res) => AppData::ok(res),
            Err(e) => {
                log::error!("登录失败: {:?}", e);
                AppData::err(500, "登录失败，请稍后再试", None)
            }
        }
    }

    ////////

    /// # 400. [API HANDLER] - 退出登录（单设备下线）
    /// * `action`: 2400 / add.out
    /// * `user_id`   从网关上下文中提取（暂不鉴权，客户端传入）
    /// * `device_id` 设备标识 — 只下线当前设备
    pub async fn handler_sign_out(cmd: PhoneLoginCommand) -> AppData<String> {
        log::info!("[API]: 收到退出登录请求: device={}", cmd.device_id);

        // 暂不鉴权，直接从 cmd 中提取 user_id 和 device_id
        // user_id 从 phone_no 字段临时传入（后续接 JWT 鉴权后可改为从 token 解析）
        let user_id: i64 = cmd.phone_no.parse().unwrap_or(0);
        if user_id <= 0 {
            return AppData::err(400, "缺少有效用户ID", None);
        }

        match AuthAddCase::case_sign_out(user_id, &cmd.device_id).await {
            Ok(msg) => AppData::ok(msg),
            Err(e) => {
                log::error!("退出登录失败: {:?}", e);
                AppData::err(500, &format!("退出失败: {}", e), None)
            }
        }
    }
}

//////// END
