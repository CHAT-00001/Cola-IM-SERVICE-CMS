// cola_user/src/api/follow/mange.rs
// core - USER - api - follow - 管理 接口
// 2026/8/2 22:21 Created.

////////

use crate::case::follow::manage::UserFollowManageCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use port::app::ctx::AppContext;
////////

/// # [API HANDLER] -  用户 关注 管理 接口
/// * `desc`: 用户关注管理 api
pub struct UserFollowManageApi;

// 构造函数
impl UserFollowManageApi {
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
        match UserFollowManageCase::case_single_del(uid, id, ctx.clone()).await {
            Ok(qty) => {
                if qty > 0 {
                    AppData::ok("✅️ Ok!".to_string()).with_msg("✅️ 单个移除关注成功")
                } else {
                    AppData::ok("⚠️ Notice".to_string()).with_msg("⚠️ 未找到该关注记录或已被移除")
                }
            }
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("❌️ 单个移除关注失败: {:?}", e),
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
        let ids: Vec<i64> = url
            .params
            .get("ids")
            .map(|s| {
                s.split(',')
                    .filter_map(|x| x.trim().parse::<i64>().ok())
                    .collect()
            })
            .unwrap_or_default();

        // Call Case
        match UserFollowManageCase::case_batch_del(uid, ids, ctx.clone()).await {
            Ok(qty) => AppData::ok(format!("✅️ 成功移除 {} 条", qty))
                .with_msg(format!("✅️ 批量移除成功，共影响 {} 条记录", qty)),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("❌️ 批量移除关注失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END