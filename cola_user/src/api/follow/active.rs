// cola_user/src/api/follow/active.rs
// core - USER - api - follow - 活跃
// 2026/8/2 22:06 Created.

////////

use cola_data::app::data::AppData;
use cola_data::app::error;

////////

/// # [ACTIVE API] -  用户 关注 活跃 接口
pub struct UserFollowActiveApi;

// 构造实现
impl UserFollowActiveApi {
    //

    ////////

    /// # 1. [API HANDLER] - 添加关注
    pub async fn api_add_follow(uid: i64, user_id: i64, is_liked: bool) -> AppData<()> {
        // 执行核心关注逻辑
        match LikeCase::case_add_follow(uid, user_id, is_liked).await {
            Ok(resp) => AppData::ok(resp).with_msg("关注成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("关注失败: {:?}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 取消关注
    pub async fn api_cancel_follow(uid: i64, user_id: i64) -> AppData<()> {
        // 执行取消关注逻辑
        match LikeCase::case_del_follow(uid, user_id, false).await {
            Ok(resp) => AppData::ok(resp).with_msg("取消关注成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("取消关注失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END
