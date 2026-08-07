// cola_user/src/api/category/state.rs
// 可乐短视频 - api - 分类 - 状态
// 2026/8/2 22:31 Created.

////////

use crate::case::category::state::UserCategoryStateCase;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::video::command::collect::CollectCommand;

////////

/// # [API HANDLER] - 用户 分类 添加 接口
pub struct UserCategorytAddApi;

// 构造函数
impl UserCategorytAddApi {
    //

    ////////

    /// # 1. [CASE] -  添加
    pub async fn api_check_state(
        uid: i64, // UID
        user_id: i64,  // 目
        query: ApiGatewayRequest, // 网关
        ctx: &AppContext,         // 全局上下文
    ) -> AppData<String> {

        match UserCategoryStateCase::case_check_state(uid, user_id, *ctx).await {
            Ok(_) => AppData::ok("收藏成功".to_string()).with_msg("收藏成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("收藏失败: {:?}", e), None),
        }
    }
}

//////// END
