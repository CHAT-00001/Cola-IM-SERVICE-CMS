// cola_user/src/api/black/get.rs
// core - USER - api - black - 获取 接口
// 2026/8/2 22:21 Created.

////////

use crate::case::black::get::UserBlackGetCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use port::ctx::AppContext;

////////

/// # [API HANDLER] -  用户 黑名单 获取 接口
pub struct UserBlackGetApi;

// 构造函数
impl UserBlackGetApi {
    //

    ////////

    /// # 1. [API HANDLER] -  获取黑名单用户列表
    pub async fn api_get_black_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<String> {
        // 1. 参数
        let offset = url.offset;
        let limit = url.limit;

        // 2. Call Case
        match UserBlackGetCase::case_get_black_list(uid, offset, limit, ctx).await {
            Ok(_) => AppData::ok("上热门成功".to_string()).with_msg("上热门成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("上热门失败: {:?}", e), None),
        }
    }
}

//////// END
