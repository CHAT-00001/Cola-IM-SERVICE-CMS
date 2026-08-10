// cola_user/src/api/black/check.rs
// 🗣 可乐用户 - api - black - 检查接口
// 2026/8/2 22:21 Created.

////////

use crate::case::black::check::UserBlackCheckCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use port::ctx::AppContext;

////////

/// # [API HANDLER] - 检查
/// * `desc`: `用户黑名单检查接口`
pub struct UserBlackCheckApi;

// 构造函数
impl UserBlackCheckApi {
    //

    ////////

    /// # 1. [API HANDLER] - 检查是否在黑名单
    pub async fn api_check_black(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<String> {
        // Call Case
        // 🗣️ CALL USER BLACK STATE
        // 注意：根据上一层 Case 返回的类型调整匹配逻辑
        match UserBlackCheckCase::case_check_black(uid, url.id, ctx).await {
            Ok(is_black) => {
                // 根据实际业务返回提示信息，例如返回 "已在黑名单" 或 "不在黑名单"
                let msg = if is_black {
                    "用户已在黑名单中"
                } else {
                    "用户不在黑名单中"
                };
                AppData::ok(is_black.to_string()).with_msg(msg)
            }
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("检查黑名单失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END
