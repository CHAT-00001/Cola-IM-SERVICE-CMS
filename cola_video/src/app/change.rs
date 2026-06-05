// cola_video/src/video/app/change.rs  -- 短视频 - 应用层 - 修改
// 2026/5/20 02:36 by wx: cestbon10080

////////

use crate::auth::app::active::ensure_user_active;
use crate::auth::port::session::SessionPort;
use crate::user::biz::permission::logic_check_user_publish_permission;
use crate::user::port::check::UserCheckPermissionPort;
use crate::user::port::user::UserPort;
use crate::video::biz;
use crate::video::port::change::ChangePort;
use crate::video::port::view::ViewPort;
use data::app::data::AppData;
use data::app::error;
use data::auth::info::auth::AuthContext;
use data::video::command::video::VideoCommand;
use data::video::entity::video::VideoEntity;
use data::video::model::video::VideoSingleResponse;

////////

/// # 1. [CASE] -  编辑视频
pub async fn case_change_edit(
    auth: &AuthContext,
    cmd: VideoCommand,
    session_port: &dyn SessionPort,
    change_port: &dyn ChangePort,
    check_port: &dyn UserCheckPermissionPort,
) -> AppData<VideoSingleResponse> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(error::UNAUTHORIZED, "未登录无法编辑视频", None),
    };

    if let Err(e) = logic_check_user_publish_permission(uid, check_port).await {
        return AppData::err(error::INTERNAL_ERROR, format!("权限检查失败: {:?}", e), None);
    }

    match biz::change::logic_change_edit(uid, cmd, change_port).await {
        Ok(_) => AppData::ok(VideoSingleResponse { info: Default::default() }).with_msg("编辑成功"),
        Err(e) => AppData::err(error::INTERNAL_ERROR, format!("编辑失败: {:?}", e), None),
    }
}

////////

/// # 2. [CASE] - 修改 - 浏览权限
pub async fn case_change_visibility_range(
    auth: &AuthContext,
    video_id: i64,
    permission: i16,
    session_port: &dyn SessionPort,
    change_port: &dyn ChangePort,
) -> AppData<bool> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
    };

    match biz::change::logic_change_visibility_range(uid, video_id, permission, change_port).await {
        Ok(_) => AppData::ok(true).with_msg("操作成功"),
        Err(e) => {
            tracing::error!("BIZ CHANGE ERROR: {:?}", e);
            AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
        }
    }
}

////////

/// # 3. [CASE] - 修改 - 评论权限
pub async fn case_change_comment_range(
    auth: &AuthContext,
    video_id: i64,
    permission: i16,
    session_port: &dyn SessionPort,
    change_port: &dyn ChangePort,
) -> AppData<bool> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
    };

    match biz::change::logic_change_comment_range(uid, video_id, permission, change_port).await {
        Ok(_) => AppData::ok(true).with_msg("操作成功"),
        Err(e) => {
            tracing::error!("BIZ CHANGE ERROR: {:?}", e);
            AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
        }
    }
}

////////

/// # 4. [CASE] - 修改 - 弹幕权限
pub async fn case_change_danmaku_range(
    auth: &AuthContext,
    video_id: i64,
    permission: i16,
    session_port: &dyn SessionPort,
    change_port: &dyn ChangePort,
) -> AppData<bool> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
    };

    match biz::change::logic_change_danmaku_range(uid, video_id, permission, change_port).await {
        Ok(_) => AppData::ok(true).with_msg("操作成功"),
        Err(e) => {
            tracing::error!("BIZ CHANGE ERROR: {:?}", e);
            AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
        }
    }
}

////////

/// # 5. [CASE] - 修改 - 下载权限
pub async fn case_change_download_range(
    auth: &AuthContext,
    video_id: i64,
    permission: i16,
    session_port: &dyn SessionPort,
    change_port: &dyn ChangePort,
) -> AppData<bool> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
    };

    match biz::change::logic_change_download_range(uid, video_id, permission, change_port).await {
        Ok(_) => AppData::ok(true).with_msg("操作成功"),
        Err(e) => {
            tracing::error!("BIZ CHANGE ERROR: {:?}", e);
            AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
        }
    }
}

////////

/// # 6. [CASE] - 修改 - 收藏权限
pub async fn case_change_collect_range(
    auth: &AuthContext,
    video_id: i64,
    permission: i16,
    session_port: &dyn SessionPort,
    change_port: &dyn ChangePort,
) -> AppData<bool> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
    };

    match biz::change::logic_change_collect_range(uid, video_id, permission, change_port).await {
        Ok(_) => AppData::ok(true).with_msg("操作成功"),
        Err(e) => {
            tracing::error!("BIZ CHANGE ERROR: {:?}", e);
            AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
        }
    }
}

////////

/// # 7. [CASE] - 修改 - 购买权限
pub async fn case_change_buy_range(
    auth: &AuthContext,
    video_id: i64,
    permission: i16,
    session_port: &dyn SessionPort,
    change_port: &dyn ChangePort,
) -> AppData<bool> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(error::UNAUTHORIZED, "未登录", None),
    };

    match biz::change::logic_change_buy_range(uid, video_id, permission, change_port).await {
        Ok(_) => AppData::ok(true).with_msg("操作成功"),
        Err(e) => {
            tracing::error!("BIZ CHANGE ERROR: {:?}", e);
            AppData::err(error::INTERNAL_ERROR, "服务器保存失败", None)
        }
    }
}

//////// END
