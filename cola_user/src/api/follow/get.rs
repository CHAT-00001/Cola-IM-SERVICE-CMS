// cola_user/api/follow/get.rs
// 用户 - api - 关注 - 获取
// 2026/8/4 02:28 Created.

////////

use crate::case::follow::get::UserFollowGetCase;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::response::ListResponse;
use cola_data::cola_user::info::user::UserInfo;
use tracing::{error, info};

////////

/// # [API HANDLER] - 用户 关注 获取 接口
pub struct UserFollowtGetApi;

// 构造函数
impl UserFollowtGetApi {
    //

    ////////

    /// # 1. [CASE] -  关注列表
    /// * `desc`: 获取关注的用户列表
    pub async fn api_get_follow_list(
        uid: i64,               // 操作者ID
        url: ApiGatewayRequest, // 网关查询参数
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<ListResponse<UserInfo>> {
        // Call Case ..
        match UserFollowGetCase::case_get_follow_list(uid, url.limit, url.offset, ctx).await {
            Ok(infos) => {
                let page_info = cola_data::app::page::PageInfo {
                    page: url.page.unwrap_or(1),
                    qty: url.qty.unwrap_or(10),
                    has_more: false,
                };
                let response = ListResponse::new(infos, page_info);
                // 打印成功日志
                info!(
                    "[🗣️ API]: ✅️ 获取关注列表成功: uid={}, count={}",
                    uid,
                    response.list.len()
                );
                AppData::ok(response).with_msg("✅️ 获取关注列表成功")
            }
            Err(e) => {
                // 打印错误日志
                error!(
                    "[🤐 API]: ❌️ 获取关注列表失败: uid={}, error={:?}",
                    uid, e
                );
                AppData::err(
                    error::INTERNAL_ERROR,
                    format!("❌️ 获取关注列表失败: {:?}", e),
                    None,
                )
            }
        }
    }
}

//////// END