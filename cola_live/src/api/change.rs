// cola_video/src/new/api/manage  -- 可乐短视频 - 接口层 - 修改
// 2026/5/20 02:36

////////

use crate::case::change::VideoChangeCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::auth::info::auth::AuthContext;

////////

/// # [API HANDLER] - 权限修改接口
pub struct ChangeApi;

// 构造函数
impl ChangeApi {
    ////////

    ////////

    /// # 2. [API HANDLER] - 修改 - 浏览权限
    pub async fn handler_set_visibility_perm(
        auth: &AuthContext,
        video_id: i64,
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

        match VideoChangeCase::case_change_visibility_perm(uid, video_id, permission).await {
            Ok(_) => AppData::ok(true).with_msg("操作成功"),
            Err(e) => {
                tracing::error!("BIZ CHANGE ERROR: {:?}", e);
                AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
            }
        }
    }

    ////////

    /// # 3. [API HANDLER] - 修改 - 评论权限
    pub async fn handler_set_comment_perm(
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

        match VideoChangeCase::case_change_comment_perm(uid, video_id, permission).await {
            Ok(_) => AppData::ok(true).with_msg("操作成功"),
            Err(e) => {
                tracing::error!("BIZ CHANGE ERROR: {:?}", e);
                AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
            }
        }
    }

    ////////

    /// # 4. [API HANDLER] - 修改 - 弹幕权限
    pub async fn handler_set_danmaku_perm(
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

        match VideoChangeCase::case_change_danmaku_perm(uid, video_id, permission).await {
            Ok(_) => AppData::ok(true).with_msg("操作成功"),
            Err(e) => {
                tracing::error!("BIZ CHANGE ERROR: {:?}", e);
                AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
            }
        }
    }

    /// # 5. [API HANDLER] - 修改 - 收藏权限
    pub async fn handler_set_collect_perm(
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

        match VideoChangeCase::case_change_collect_perm(uid, video_id, permission).await {
            Ok(_) => AppData::ok(true).with_msg("操作成功"),
            Err(e) => {
                tracing::error!("BIZ CHANGE ERROR: {:?}", e);
                AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
            }
        }
    }

    ////////

    /// # 6. [API HANDLER] - 修改 - 下载权限
    pub async fn handler_set_download_perm(
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

        match VideoChangeCase::case_change_download_perm(uid, video_id, permission).await {
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

        match VideoChangeCase::case_change_buy_perm(uid, video_id, permission).await {
            Ok(_) => AppData::ok(true).with_msg("操作成功"),
            Err(e) => {
                tracing::error!("BIZ CHANGE ERROR: {:?}", e);
                AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
            }
        }
    }
}

//////// END
