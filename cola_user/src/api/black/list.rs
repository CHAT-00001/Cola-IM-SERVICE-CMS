// cola_user/src/api/black/list.rs
// 用户 - api - black - 列表接口
// 2026/8/2 22:21 Created.

////////

use crate::case::black::list::UserBlackListCase;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;

////////

/// # [API HANDLER] - 列表
/// * `desc`: `用户黑名单列表接口`
pub struct UserBlackListApi;

// 构造函数
impl UserBlackListApi {
    //

    ////////

    /// # 1. [API HANDLER] - 我的
    /// * `desc`: `获取我的黑名单`
    pub async fn api_get_my_black_list(
        uid: i64,               // UID
        url: ApiGatewayRequest, // 网关
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<String> {
        // Call Case
        // 🗣️ CALL USER BLACK STATE
        // 注意：根据上一层 Case 返回的类型调整匹配逻辑
        match UserBlackListCase::case_get_my_black_list(uid, url.id, url.limit, url.offset, ctx)
            .await
        {
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
