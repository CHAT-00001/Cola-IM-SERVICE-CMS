// // cola_auth/src/app/active.rs  -- AUTH app 状态
// // 2026/04/13 10:15 by wx: cestbon10080
//
// //////
//
// use cola_data::app::data::AppData;
// use cola_data::app::error;
// use cola_data::auth::info::auth::AuthContext;
// use crate::model::vo::session::SessionResponse;
// use crate::port::session::SessionPort;
//
//
// //////
//
// /// # [CASE] -  ensure_user_active 用户状态守卫
// /// 从验证中心（Auth）校验 Token 并确保用户状态正常
// pub async fn ensure_user_active(
//     query: &AuthContext,
//     auth_port: &dyn SessionPort,
// ) -> AppData<SessionResponse> {
//
//     // 1. 提取 Token
//     let token = if !query.access_token.is_empty() {
//         &query.access_token
//     } else {
//         return AppData::err(error::UNAUTHORIZED, "请先登录", None);
//     };
//
//     // 2. 通过 SessionPort 进行校验
//     let session_opt = match auth_port.get_session(token).await {
//         Ok(opt) => opt,
//         Err(e) => {
//             tracing::error!("AUTH_PORT ERROR: {:?}", e);
//             return AppData::err(error::INTERNAL_ERROR, "验证服务暂时不可用", None);
//         }
//     };
//
//     // 3. 检查会话是否存在
//     let session = match session_opt {
//         Some(s) => s,
//         None => return AppData::err(error::UNAUTHORIZED, "登录状态已过期", None),
//     };
//
//     // 4. 账号状态检查
//     if session.user_info.status == 0 {
//         return AppData::err(error::FORBIDDEN, "当前账号已被冻结，请联系客服", None);
//     }
//
//     // 5. 校验通过，返回完整会话数据
//     AppData::ok(session)
// }
//
// /// # UseCase: get_auth_uid
// /// 辅助函数：快速获取受保护的 UID (i64)
// pub async fn get_auth_uid(
//     auth: &AuthContext,
//     auth_port: &dyn SessionPort,
// ) -> Result<i64, AppData<i64>> {
//     // 调用上面的校验函数
//     match ensure_user_active(auth, auth_port).await {
//         // 如果成功获取到 session
//         AppData { data: Some(session), .. } => {
//             // 从 session.info.token.key 获取用户 ID
//             let user_id = session.info.token.key.parse::<i64>().unwrap_or(0);
//             Ok(user_id)
//         },
//         // 如果校验失败，透传错误信息（包括错误码和提示信息）
//         err_data => Err(err_data.rebind()),
//     }
// }
