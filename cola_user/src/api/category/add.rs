// user/src/api/category/active
// core - USER - api - category - 添加 接口
// 2026/8/2 22:18 Created.

////////

use crate::case::category::add::UserCategoryAddCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use port::app::ctx::AppContext;
use tracing::{error, info};

////////

/// # [API HANDLER] -  添加接口
pub struct UserCategoryAddApi;

// 构造函数
impl UserCategoryAddApi {
    //

    ////////

    /// # 1. [API HANDLER] -  添加分类
    pub async fn api_add_category(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<String> {
        // 目标用户ID
        let id = url.id;

        // Call Case
        match UserCategoryAddCase::case_add_category(uid, id, ctx).await {
            Ok(_) => {
                // 打印成功日志到控制台
                info!(
                    "[🗣️ API]: ✅️ 添加黑名单成功: 操作者uid={}, 目标id={}",
                    uid, id
                );
                AppData::ok("✅️ 添加黑名单成功".to_string()).with_msg("✅️ 添加黑名单成功")
            }
            Err(e) => {
                // 打印错误日志到控制台
                error!(
                    "[🤐 API]: ❌️ 添加黑名单失败: 操作者uid={}, 目标id={}, 错误: {:?}",
                    uid, id, e
                );
                AppData::err(
                    error::INTERNAL_ERROR,
                    format!("❌️ 添加黑名单失败: {:?}", e),
                    None,
                )
            }
        }
    }

    ////////

    /// # 2. [API HANDLER] -  移除黑名单
    pub async fn api_unblock_category(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<String> {
        // 目标用户ID
        let id = url.id;

        // Call Case
        match UserCategoryAddCase::case_del_category(uid, id, ctx).await {
            Ok(_) => {
                // 打印成功日志到控制台
                info!(
                    "[🗣️ API]: ✅️ 移除黑名单成功: 操作者uid={}, 目标id={}",
                    uid, id
                );
                AppData::ok("✅️ 移除黑名单成功".to_string()).with_msg("✅️ 移除黑名单成功")
            }
            Err(e) => {
                // 打印错误日志到控制台
                error!(
                    "[🤐 API]: ❌️ 移除黑名单失败: 操作者uid={}, 目标id={}, 错误: {:?}",
                    uid, id, e
                );
                AppData::err(
                    error::INTERNAL_ERROR,
                    format!("❌️ 移除黑名单失败: {:?}", e),
                    None,
                )
            }
        }
    }
}

//////// END
