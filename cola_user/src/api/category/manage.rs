// cola_user/src/api/category/manage.rs
// core - USER - api - 分类 - 管理 接口
// 2026/8/2 22:21 Created.

////////

use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use crate::case::category::manage::UserCategoryManageCase;
////////

/// # [API HANDLER] -  用户 黑名单 管理 接口
pub struct UserCategoryManageApi;

// 构造函数
impl UserCategoryManageApi {
    //

    ////////

    /// # 1. [API HANDLER] -  单个移除
    pub async fn api_single_del(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<String> {
        let id = url.user_id;

        // Call Case
        match UserCategoryManageCase::case_single_del(uid, id, ctx.clone()).await {
            Ok(qty) => {
                if qty > 0 {
                    AppData::ok("✅️ Ok!".to_string()).with_msg("✅️ 单个移除黑名单成功")
                } else {
                    AppData::ok("⚠️ Notice".to_string()).with_msg("⚠️ 未找到该黑名单记录或已被移除")
                }
            }
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("❌️ 单个移除黑名单失败: {:?}", e),
                None,
            ),
        }
    }

    ////////

    /// # 2. [API HANDLER] -  批量移除
    pub async fn api_batch_del(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<String> {
        let ids = url.params.ids;

        // Call Case
        match UserCategoryManageCase::case_batch_del(uid, ids, ctx.clone()).await {
            Ok(qty) => {
                AppData::ok(format!("✅️ 成功移除 {} 条", qty))
                    .with_msg(format!("✅️ 批量移除成功，共影响 {} 条记录", qty))
            }
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("❌️ 批量移除黑名单失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END