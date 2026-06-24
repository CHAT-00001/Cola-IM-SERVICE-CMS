// cola_auth/src/api/add.rs  -- 可乐验证中心 - 接口层 - 登录
// 2026/6/9 07:54

////////

use crate::case::session::{SessionCase};
use crate::model::vo::session::SessionResponse;
use cola_data::app::api::ApiQuery;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::auth::command::phone::PhoneLoginCommand;
use tracing::log;
use validator::Validate;
use cola_data::auth::command::email::EmailLoginCommand;
use crate::case::add::AuthAddCase;
use crate::case::login::LoginCase;

////////

/// # [API] - 登录 接口
pub struct AuthAddApi;

// 构造函数
impl AuthAddApi {

    ////////

    /// # 1. [CASE] - 手机短信验证码登录
    /// * `action`: 2001
    /// * `desc`: 手机 + 短信
    pub async fn handler_sign_in_by_phone(cmd: PhoneLoginCommand) -> AppData<SessionResponse> {
        // 1. 校验 (输入端边界检查)
        if let Err(e) = cmd.validate_params() {
            return AppData::err(400, &format!("参数校验失败: {}", e), None);
        }

        // 2. 调度业务逻辑 (Biz 层)
        // 注意：这里不需要手动处理数据库事务，Biz 层会处理
        match AuthAddCase::case_sign_in_by_phone(&cmd).await {
            Ok(res) => {
                // 3. 封装成功结果
                AppData::ok(res)
            }
            Err(e) => {
                // 4. 异常处理 (区分用户错误与系统错误)
                // 记录日志，但不把内部错误细节直接抛给客户端
                log::error!("登录失败: {:?}", e);
                AppData::err(500, "登录失败，请稍后再试", None)
            }
        }
    }

    ////////

    /// # 2. [CASE] - 邮箱验证码登录
    /// * `action`: 2002
    /// * `desc`: email + code
    pub async fn handler_sign_in_by_email(cmd: EmailLoginCommand) -> AppData<SessionResponse> {
        // 1. 校验 (输入端边界检查)
        if let Err(e) = cmd.validate_params() {
            return AppData::err(400, &format!("参数校验失败: {}", e), None);
        }

        // 2. 调度业务逻辑 (Biz 层)
        // 注意：这里不需要手动处理数据库事务，Biz 层会处理
        match AuthAddCase::case_sign_in_by_email(&cmd).await {
            Ok(res) => {
                // 3. 封装成功结果
                AppData::ok(res)
            }
            Err(e) => {
                // 4. 异常处理 (区分用户错误与系统错误)
                // 记录日志，但不把内部错误细节直接抛给客户端
                log::error!("登录失败: {:?}", e);
                AppData::err(500, "登录失败，请稍后再试", None)
            }
        }
    }

    ////////

    /// # 3. [CASE] - 账号密码登录
    /// * `action`: 2003
    /// * `desc`: ac + pw + code
    pub async fn handler_sign_in_by_pwd(cmd: PhoneLoginCommand) -> AppData<SessionResponse> {
        // 1. 校验 (输入端边界检查)
        if let Err(e) = cmd.validate_params() {
            return AppData::err(400, &format!("参数校验失败: {}", e), None);
        }

        // 2. 调度业务逻辑 (Biz 层)
        // 注意：这里不需要手动处理数据库事务，Biz 层会处理
        match LoginCase::case_phone(&cmd).await {
            Ok(res) => {
                // 3. 封装成功结果
                AppData::ok(res)
            }
            Err(e) => {
                // 4. 异常处理 (区分用户错误与系统错误)
                // 记录日志，但不把内部错误细节直接抛给客户端
                log::error!("登录失败: {:?}", e);
                AppData::err(500, "登录失败，请稍后再试", None)
            }
        }
    }

    ////////

    /// # 4. [CASE] - 谷歌登录
    /// * `desc`: 2004
    pub async fn handler_sign_in_by_google(cmd: PhoneLoginCommand) -> AppData<SessionResponse> {
        // 1. 校验 (输入端边界检查)
        if let Err(e) = cmd.validate_params() {
            return AppData::err(400, &format!("参数校验失败: {}", e), None);
        }

        // 2. 调度业务逻辑 (Biz 层)
        // 注意：这里不需要手动处理数据库事务，Biz 层会处理
        match LoginCase::case_phone(&cmd).await {
            Ok(res) => {
                // 3. 封装成功结果
                AppData::ok(res)
            }
            Err(e) => {
                // 4. 异常处理 (区分用户错误与系统错误)
                // 记录日志，但不把内部错误细节直接抛给客户端
                log::error!("登录失败: {:?}", e);
                AppData::err(500, "登录失败，请稍后再试", None)
            }
        }
    }

    ////////

    /// # 5. [CASE] - 苹果登录
    /// * `action`: 2005
    pub async fn handler_sign_in_by_apple(cmd: PhoneLoginCommand) -> AppData<SessionResponse> {
        // 1. 校验 (输入端边界检查)
        if let Err(e) = cmd.validate_params() {
            return AppData::err(400, &format!("参数校验失败: {}", e), None);
        }

        // 2. 调度业务逻辑 (Biz 层)
        // 注意：这里不需要手动处理数据库事务，Biz 层会处理
        match LoginCase::case_phone(&cmd).await {
            Ok(res) => {
                // 3. 封装成功结果
                AppData::ok(res)
            }
            Err(e) => {
                // 4. 异常处理 (区分用户错误与系统错误)
                // 记录日志，但不把内部错误细节直接抛给客户端
                log::error!("登录失败: {:?}", e);
                AppData::err(500, "登录失败，请稍后再试", None)
            }
        }
    }

    ////////

    /// # 6. [CASE] - 微信登录
    /// * `action`: 2006
    pub async fn handler_sign_in_by_wechat(cmd: PhoneLoginCommand) -> AppData<SessionResponse> {
        // 1. 校验 (输入端边界检查)
        if let Err(e) = cmd.validate_params() {
            return AppData::err(400, &format!("参数校验失败: {}", e), None);
        }

        // 2. 调度业务逻辑 (Biz 层)
        // 注意：这里不需要手动处理数据库事务，Biz 层会处理
        match LoginCase::case_phone(&cmd).await {
            Ok(res) => {
                // 3. 封装成功结果
                AppData::ok(res)
            }
            Err(e) => {
                // 4. 异常处理 (区分用户错误与系统错误)
                // 记录日志，但不把内部错误细节直接抛给客户端
                log::error!("登录失败: {:?}", e);
                AppData::err(500, "登录失败，请稍后再试", None)
            }
        }
    }


}

//////// END


