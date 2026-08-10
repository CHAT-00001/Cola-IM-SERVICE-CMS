// cola_user/src/api/follow/active.rs
// core - USER - api - follow - 活跃
// 2026/8/2 22:06 Created.

////////

use crate::case::follow::add::UserFollowAddCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use port::ctx::AppContext;

////////

/// # [ACTIVE API] -  用户 关注 活跃 接口
pub struct UserFollowActiveApi;

// 构造实现
impl UserFollowActiveApi {
    //

    ////////

    /// # 1. [API HANDLER] - 添加关注
    pub async fn api_add_follow(uid: i64, user_id: i64, ctx: &AppContext) -> AppData<()> {
        // 执行核心关注逻辑
        match UserFollowAddCase::case_add_follow(uid, user_id, ctx).await {
            Ok(_) => AppData::ok(()).with_msg("关注成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("关注失败: {:?}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 取消关注
    pub async fn api_cancel_follow(uid: i64, user_id: i64, ctx: &AppContext) -> AppData<()> {
        // 执行取消关注逻辑
        match UserFollowAddCase::case_remove_follow(uid, user_id, ctx).await {
            Ok(_) => AppData::ok(()).with_msg("取消关注成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("取消关注失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END
