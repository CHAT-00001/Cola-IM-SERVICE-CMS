// // auth/src/api/service/get.rs  -- AUTH - api - 会话 - 获取接口
// // 2026/6/9 07:54 Created.
//
// ////////
//
// use crate::case::binding::LoginCase;
// use crate::case::service::add::AuthAddCase;
// use cola_data::app::api::ApiQuery;
// use cola_data::app::data::AppData;
// use cola_data::app::error;
// use cola_data::auth::cmd::email::EmailLoginCommand;
// use cola_data::auth::cmd::phone::PhoneLoginCommand;
// use cola_data::auth::cmd::service::refresh::SessionRefreshCommand;
// use cola_data::auth::vo::service::SignResponse;
// use port::app::ctx::AppContext;
// use tracing::log;
// use validator::Validate;
// use cola_data::auth::cmd::service::SessionCommand;
// ////////
//
// /// # [GET API] - 验证会话获取接口
// pub struct SessionGetApi;
//
// impl SessionGetApi {
//     //
//
//     ////////
//
//     /// # 1. [API HANDLER] - 客户端刷新access_token
//     /// * `desc`: `refresh_token + device_id`
//     ///
//     pub async fn get_session(
//         uid: i64,                   // 操作者 ID
//         cmd: SessionCommand, // 刷新命令
//         ctx: &AppContext,           // 应用上下文
//     ) -> AppData<SignResponse> {
//         log::info!("[API]: 收到 📱 手机号登录原始命令数据: {:?}", cmd);
//
//         // 1. 校验
//         if let Err(e) = cmd.validate_params() {
//             return AppData::err(4000, &format!("参数校验失败: {}", e), None);
//         }
//
//         // 2. 调度业务逻辑
//         match AuthAddCase::case_sign_in_by_phone(&cmd, ctx).await {
//             Ok(res) => AppData::ok(res),
//             Err(e) => {
//                 log::error!("登录失败: {:?}", e);
//                 AppData::err(5000, "登录失败，请稍后再试", None)
//             }
//         }
//     }
//
//     ////////
// }
//
// //////// END
