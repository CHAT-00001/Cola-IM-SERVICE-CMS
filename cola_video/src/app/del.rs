// cola_video/src/video/app/del  -- 核心 - 短视频 - 应用层 - 删除
// 2026/5/20 02:46 by wx: cestbon10080
// * 4个函数
// * --------

////////

use crate::auth::app::active::ensure_user_active;
use crate::auth::port::session::SessionPort;
use crate::user::port::check::UserCheckPermissionPort;
use crate::video::biz;
use crate::video::port::del::DelPort;
use data::app::data::AppData;
use data::app::error;
use data::auth::info::auth::AuthContext;

////////

/// # 1. [CASE] - 删除 - 视频
pub async fn case_del_publish(
    auth: &AuthContext,
    video_id: i64,
    session_port: &dyn SessionPort,
    del_port: &dyn DelPort,
) -> AppData<bool> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(error::UNAUTHORIZED, "登录状态已失效", None),
    };

    match biz::del::logic_del_video(uid, video_id, del_port).await {
        Ok(_) => AppData::ok(true).with_msg("操作成功"),
        Err(e) => {
            tracing::error!("DELETE VIDEO ERROR: {:?}", e);
            AppData::err(error::INTERNAL_ERROR, "删除失败", None)
        }
    }
}

////////

/// # 2. [CASE] - 删除 - 评论
pub async fn case_del_comment(
    auth: &AuthContext,
    comment_id: i64,
    session_port: &dyn SessionPort,
    del_port: &dyn DelPort,
) -> AppData<bool> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(error::UNAUTHORIZED, "未登录无法删除评论", None),
    };

    match biz::del::logic_del_comment(uid, comment_id, del_port).await {
        Ok(_) => AppData::ok(true).with_msg("操作成功"),
        Err(e) => {
            tracing::error!("DELETE COMMENT ERROR: {:?}", e);
            AppData::err(error::INTERNAL_ERROR, "删除失败", None)
        }
    }
}

////////

/// # 3. [CASE] - 删除 - 弹幕
pub async fn case_del_danmaku(
    auth: &AuthContext,
    danmaku_id: i64,
    session_port: &dyn SessionPort,
    del_port: &dyn DelPort,
) -> AppData<bool> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(error::UNAUTHORIZED, "登录状态已失效", None),
    };

    match biz::del::logic_del_danmaku(uid, danmaku_id, del_port).await {
        Ok(_) => AppData::ok(true).with_msg("操作成功"),
        Err(e) => {
            tracing::error!("DELETE DANMAKU ERROR: {:?}", e);
            AppData::err(error::INTERNAL_ERROR, "删除失败", None)
        }
    }
}

////////

/// # 4. [CASE] - 删除 - 收藏
pub async fn case_del_collect(
    auth: &AuthContext,
    collect_id: i64,
    session_port: &dyn SessionPort,
    del_port: &dyn DelPort,
) -> AppData<bool> {
    let auth_res = ensure_user_active(auth, session_port).await;
    if auth_res.code != 0 {
        return AppData::err(auth_res.code, auth_res.message, None);
    }

    let uid = match auth.user_id {
        Some(id) => id,
        None => return AppData::err(error::UNAUTHORIZED, "登录状态已失效", None),
    };

    match biz::del::logic_del_collect(uid, collect_id, del_port).await {
        Ok(_) => AppData::ok(true).with_msg("操作成功"),
        Err(e) => {
            tracing::error!("DELETE COLLECT ERROR: {:?}", e);
            AppData::err(error::INTERNAL_ERROR, "删除失败", None)
        }
    }
}

//////// END
