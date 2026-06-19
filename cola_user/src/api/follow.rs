// cola_user/src/api/follow.rs  -- USER - 接口层 - 关注
// 2026/4/12 14:45

//////

use cola_data::app::data::AppData;
use cola_data::app::error;
use crate::case::follow::LikeCase;

//////

/// # [FOLLOW CASE] -  关注 接口
pub struct LikeApi;

impl LikeApi {

    ////////

    /// # 1. [API HANDLER] - 添加关注
    pub async fn handler_add_follow(
        uid: i64,
        user_id: i64,
        is_liked: bool,
    ) -> AppData<()> {

        // 执行核心关注逻辑
        match LikeCase::case_add_follow(uid, user_id, is_liked).await {
            Ok(resp) => AppData::ok(resp).with_msg("关注成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("关注失败: {:?}", e), None),
        }
    }


    ////////

    /// # 2. [API HANDLER] - 取消关注
    pub async fn handler_del_follow(
        uid: i64,
        user_id: i64,
    ) -> AppData<()> {

        // 执行取消关注逻辑
        match LikeCase::case_del_follow(uid, user_id, false).await {
            Ok(resp) => AppData::ok(resp).with_msg("取消关注成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("取消关注失败: {:?}", e), None),
        }
    }

}

//////// END
