// user/api/follow/get.rs
// 用户 - api - 关注 - 获取
// 2026/8/4 02:28 Created.

////////

use crate::case::follow::get::UserFollowGetCase;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::response::ListResponse;
use cola_data::user::info::user::UserInfo;
use tracing::{error, info};

////////

/// # [API HANDLER] - 用户 分类 获取 接口
pub struct UserFollowtGetApi;

// 构造函数
impl UserFollowtGetApi {
    //

    ////////

    /// # 1. [CASE] -  最新
    /// * `desc`: 获取最新的分类列表
    pub async fn api_get_new_category_list(
        uid: i64,               // 操作者ID
        url: ApiGatewayRequest, // 网关查询参数
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<ListResponse<UserInfo>> {
        // Call Case ..
        match UserFollowGetCase::case_get_new_list(uid, url.limit, url.offset, ctx).await {
            Ok(response) => {
                // 打印成功日志
                info!(
                    "[🗣️ API]: ✅️ 获取最新的用户分类成功: uid={}, count={}",
                    uid,
                    response.list.len()
                );
                AppData::ok(response).with_msg("✅️ 获取最新的用户分类成功")
            }
            Err(e) => {
                // 打印错误日志
                error!(
                    "[🤐 API]: ❌️ 获取最新的用户分类失败: uid={}, error={:?}",
                    uid, e
                );
                AppData::err(
                    error::INTERNAL_ERROR,
                    format!("❌️ 获取最新的用户分类失败: {:?}", e),
                    None,
                )
            }
        }
    }
}

//////// END
