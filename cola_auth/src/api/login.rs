// cola_auth/src/api/port.rs  -- 可乐验证中心 - 接口层 - 登录
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
use crate::case::login::LoginCase;

////////

/// # [API] - 登录接口
pub struct LoginApi;

// 构造函数
impl LoginApi {

    ////////

    /// # 1. [APP USE CASE] - 手机验证码登录
    /// * action: 1001
    pub async fn login_by_phone(cmd: PhoneLoginCommand) -> AppData<SessionResponse> {
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

    /// # 2. [APP USE CASE] - 邮箱验证码登录
    /// * action: 1002
    pub async fn login_by_email(cmd: PhoneLoginCommand) -> AppData<SessionResponse> {
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

    /// # 3. [APP USE CASE] - 账号密码登录
    /// * action: 1003
    pub async fn login_by_pw(cmd: PhoneLoginCommand) -> AppData<SessionResponse> {
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

    /// # 4. [APP USE CASE] - 谷歌登录
    /// * action: 1004
    pub async fn login_by_google(cmd: PhoneLoginCommand) -> AppData<SessionResponse> {
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

    /// # 5. [APP USE CASE] - 苹果登录
    /// * action: 1005
    pub async fn login_by_apple(cmd: PhoneLoginCommand) -> AppData<SessionResponse> {
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

    /// # 6. [APP USE CASE] - 微信登录
    /// * action: 1006
    pub async fn login_by_wechat(cmd: PhoneLoginCommand) -> AppData<SessionResponse> {
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

    /// # [APP USE CASE] - 发送短信验证码
    /// * action: 6001
    pub async fn case_send_sms_code(phone: &String) -> AppData<String> {
        // 1. 基础校验
        if phone.is_empty() {
            return AppData::err(400, "手机号不能为空", None);
        }

        // 2. 🚀 剥离 Port，静态下钻到本地的验证码 Biz
        match crate::case::sms::logic_send_sms_code(phone).await {
            Ok(_) => AppData::ok("验证码发送成功".to_string()),
            Err(e) => AppData::err(500, &e.to_string(), None),
        }
    }

    ////////

    /// # [APP USE CASE] - 发送邮箱验证码
    /// * action: 6002
    pub async fn case_send_sms(email: &String) -> AppData<String> {
        // 1. 基础校验
        if email.is_empty() {
            return AppData::err(400, "邮箱不能为空", None);
        }

        // 2. 🚀 剥离 Port，静态下钻到本地的验证码 Biz
        match crate::case::sms::logic_send_sms_code(email).await {
            Ok(_) => AppData::ok("验证码发送成功".to_string()),
            Err(e) => AppData::err(500, &e.to_string(), None),
        }
    }

    ////////

}

//////// END


