// cola_user/src/api/change.rs  -- 用户中心 - 接口层 - 修改
// 2026/5/20 02:36

////////

use crate::case::change::ChangeCase;
use crate::case::comment::CommentCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::auth::info::auth::AuthContext;

////////

/// # [CHANGE API] - 修改 接口
pub struct ChangeApi;

// 构造函数
impl ChangeApi {
    ////////

    /// # 2. [API HANDLER] - 修改 - 昵称
    pub async fn handler_nickname(
        auth: &AuthContext,
        user_id: i64,
        permission: i16,
    ) -> AppData<bool> {
        //
        // // 1. 检查视频修改权限
        // let auth_res = ensure_user_active(auth, session_port).await;
        // if auth_res.code != 0 {
        //     return AppData::err(auth_res.code, auth_res.message, None);
        // }
        //
        // let uid = match auth.user_id {
        //     Some(id) => id,
        //     None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
        // };

        let uid = 10048;

        match ChangeCase::case_change_nickname(uid, user_id, permission).await {
            Ok(_) => AppData::ok(true).with_msg("操作成功"),
            Err(e) => {
                tracing::error!("BIZ CHANGE ERROR: {:?}", e);
                AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
            }
        }
    }

    ////////

    /// # 3. [API HANDLER] - 修改 - 个签
    pub async fn handler_signature(
        auth: &AuthContext,
        user_id: i64,
        permission: i16,
    ) -> AppData<bool> {
        // // 1. 检查视频修改权限
        // let auth_res = ensure_user_active(auth, session_port).await;
        // if auth_res.code != 0 {
        //     return AppData::err(auth_res.code, auth_res.message, None);
        // }
        //
        // let uid = match auth.user_id {
        //     Some(id) => id,
        //     None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
        // };

        let uid = 10048;

        match ChangeCase::case_change_signature(uid, user_id, permission).await {
            Ok(_) => AppData::ok(true).with_msg("操作成功"),
            Err(e) => {
                tracing::error!("BIZ CHANGE ERROR: {:?}", e);
                AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
            }
        }
    }

    ////////

    /// # 4. [API HANDLER] - 修改 - 头像
    pub async fn handler_avatar(
        auth: &AuthContext,
        user_id: i64,
        permission: i16,
    ) -> AppData<bool> {
        // // 1. 检查视频修改权限
        // let auth_res = ensure_user_active(auth, session_port).await;
        // if auth_res.code != 0 {
        //     return AppData::err(auth_res.code, auth_res.message, None);
        // }
        //
        // let uid = match auth.user_id {
        //     Some(id) => id,
        //     None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
        // };
        let uid = 10048;

        match ChangeCase::case_change_avatar(uid, user_id, permission).await {
            Ok(_) => AppData::ok(true).with_msg("操作成功"),
            Err(e) => {
                tracing::error!("BIZ CHANGE ERROR: {:?}", e);
                AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
            }
        }
    }

    /// # 5. [API HANDLER] - 修改 - 背景图
    pub async fn handler_bg_img(
        auth: &AuthContext,
        user_id: i64,
        permission: i16,
    ) -> AppData<bool> {
        // // 1. 检查视频修改权限
        // let auth_res = ensure_user_active(auth, session_port).await;
        // if auth_res.code != 0 {
        //     return AppData::err(auth_res.code, auth_res.message, None);
        // }
        //
        // let uid = match auth.user_id {
        //     Some(id) => id,
        //     None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
        // };

        let uid = 10048;

        match ChangeCase::case_change_bg_img(uid, user_id, permission).await {
            Ok(_) => AppData::ok(true).with_msg("操作成功"),
            Err(e) => {
                tracing::error!("BIZ CHANGE ERROR: {:?}", e);
                AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
            }
        }
    }

    ////////

    /// # 6. [API HANDLER] - 修改 - 生日
    pub async fn handler_birthday(
        auth: &AuthContext,
        user_id: i64,
        permission: i16,
    ) -> AppData<bool> {
        // // 1. 检查视频修改权限
        // let auth_res = ensure_user_active(auth, session_port).await;
        // if auth_res.code != 0 {
        //     return AppData::err(auth_res.code, auth_res.message, None);
        // }
        //
        // let uid = match auth.user_id {
        //     Some(id) => id,
        //     None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
        // };

        let uid = 10048;

        match ChangeCase::case_change_birthday(uid, user_id, permission).await {
            Ok(_) => AppData::ok(true).with_msg("操作成功"),
            Err(e) => {
                tracing::error!("BIZ CHANGE ERROR: {:?}", e);
                AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
            }
        }
    }

    ////////

    /// # 7. [API HANDLER] - 修改 - 购买权限
    pub async fn handler_set_buy_perm(
        auth: &AuthContext,
        video_id: i64,
        permission: i16,
    ) -> AppData<bool> {
        // // 1. 检查视频修改权限
        // let auth_res = ensure_user_active(auth, session_port).await;
        // if auth_res.code != 0 {
        //     return AppData::err(auth_res.code, auth_res.message, None);
        // }
        //
        // let uid = match auth.user_id {
        //     Some(id) => id,
        //     None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
        // };

        let uid = 10048;

        match ChangeCase::case_change_buy_perm(uid, video_id, permission).await {
            Ok(_) => AppData::ok(true).with_msg("操作成功"),
            Err(e) => {
                tracing::error!("BIZ CHANGE ERROR: {:?}", e);
                AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
            }
        }
    }
}

//////// END
