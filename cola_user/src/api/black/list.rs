// cola_user/src/api/black/list.rs
// 用户 - api - black - 列表接口
// 2026/8/2 22:21 Created.

////////

use crate::case::black::list::UserBlackListCase;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::page::PageInfo;
use cola_data::app::response::ListResponse;
use cola_data::cola_user::info::user::UserInfo;
use tracing::{error, info};

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
    ) -> AppData<ListResponse<UserInfo>> {
        // Call Case
        match UserBlackListCase::case_get_my_black_list(uid, url.id, url.limit, url.offset, ctx)
            .await
        {
            Ok(infos) => {
                info!("[🗣️ API]: ✅️ 获取我的黑名单成功: uid={}, count={}", uid, infos.len());
                let page_info = PageInfo {
                    page: url.page.unwrap_or(1),
                    qty: url.qty.unwrap_or(10),
                    has_more: false,
                };
                let response = ListResponse::new(infos, page_info);
                AppData::ok(response).with_msg("✅️ 获取我的黑名单成功")
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