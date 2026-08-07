// cola_video/src/api/comment/report.rs
// 视频 - 应用层 - 评论 - 举报
// 2026/8/4 18:46 Created.

////////

use crate::case::comment::CommentCase;
use crate::model::vo::comment::{CommentListResponse, CommentSingleResponse};
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::cola_auth::info::auth::AuthContext;
use cola_data::cola_video::command::comment::CommentCommand;
use repository::cola_user::service::state::UserStateService;

////////

#[derive(Clone)]
pub struct CommentParamsQuery {
    pub video_id: i64,
    pub comment_id: i64,
    pub is_unliked: bool,
    pub snowflake: i64,
}

////////

/// # [ADD API]
/// * `desc`: `评论举报接口`
pub struct VideoCommentReportApi;

impl VideoCommentReportApi {
    // 💡

    ////////

    /// # 1. [API HANDLER] - 发布
    pub async fn handler_add_comment(
        auth: AuthContext, // 验证中心
        url: CommentParamsQuery,
        cmd: CommentCommand, // 评论命令
        ctx: &AppContext,    // 🌟 核心修复：把业务上下文注入进来
    ) -> AppData<CommentSingleResponse> {
        // 1. 检查用户状态

        // 2. 检查评论权限
        let video_id = url.video_id;

        // 3. 调用CASE (补充传入 ctx)
        match CommentCase::case_add_comment(auth.uid, video_id, cmd, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("New Videos Error: {:?}", e);
                AppData::err(5001, "发布评论失败", None)
            }
        }
    }

    ////////

    /// # 2. [API HANDLER] - 视频的评论
    pub async fn handler_view_video_list(
        auth: AuthContext,
        video_id: Option<i64>,
        query: ApiUrlParamsQuery,
        ctx: &AppContext,
    ) -> AppData<CommentListResponse> {
        // 1. 参数校验：获取 uid
        let uid = auth.uid;

        // 2. 参数校验：video_id
        let video_id = match video_id {
            Some(id) => id,
            None => {
                return AppData::err(4002, "缺少video_id参数", None);
            }
        };

        // 3. 主流程
        match CommentCase::case_get_video_comments_list(uid, video_id, query, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Comment error: {:?}", e);
                AppData::err(5001, "获取视频的评论列表失败", None)
            }
        }
    }

    ////////

    /// # 3. [API HANDLER] - 查看用户的评论
    pub async fn handler_view_user_list(
        auth: AuthContext,
        _url: CommentParamsQuery,
        query: ApiUrlParamsQuery,
        ctx: &AppContext,
    ) -> AppData<CommentListResponse> {
        let uid = auth.uid;

        // 2. Call Service 检查用户状态
        if let Err(_) = UserStateService::check_user_valid(uid).await {
            return AppData::err(4004, "用户不存在或者已被删除", None);
        }

        // 3. 主流程 —— 🌟 核心修复：Some(uid) 改成 uid，并在末尾追加 ctx
        match CommentCase::case_get_user_comments_list(uid, query, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Comment error: {:?}", e);
                AppData::err(5001, "获取用户的评论列表失败", None)
            }
        }
    }

    /// # 4. [API HANDLER] - 单条删除
    pub async fn handler_del_comment(
        auth: AuthContext,
        url: CommentParamsQuery,
        ctx: &AppContext,
    ) -> AppData<String> {
        // 🌟 核心修复：尊重客观事实，类型改成 String

        // 1. 参数校验：comment_id
        let comment_id = match url.comment_id {
            0 => {
                return AppData::err(4002, "缺少comment_id参数", None);
            }
            id => id,
        };

        // 3. Call Case: 删除一条评论
        match CommentCase::case_del_one_item(auth.uid, comment_id, url.snowflake, ctx).await {
            Ok(resp) => AppData::ok(resp), // ✅ 现在的 resp (String) 完美契合 AppData<String>
            Err(e) => {
                tracing::error!("Comment error: {:?}", e);
                AppData::err(6001, "单条评论删除失败", None)
            }
        }
    }

    ////////

    /// # 5. [API HANDLER] - 点赞
    pub async fn handler_add_like(
        auth: AuthContext,
        url: CommentParamsQuery,
        is_liked: bool,
        ctx: &AppContext,
    ) -> AppData<bool> {
        let uid = auth.uid;

        let comment_id = match url.comment_id {
            0 => return AppData::err(4002, "缺少comment_id", None),
            id => id,
        };

        // CALL CASE
        match CommentCase::case_add_comment_like(uid, comment_id, is_liked).await {
            Ok(_) => AppData::ok(true),
            Err(e) => {
                tracing::error!("like comment error: {:?}", e);
                AppData::err(5001, "点赞失败", None)
            }
        }
    }

    ////////

    /// # 6. [API HANDLER] - 不喜欢
    pub async fn handler_add_unlike(
        auth: AuthContext,
        url: CommentParamsQuery,
        ctx: &AppContext, // 🌟 已经在这里了
    ) -> AppData<()> {
        // CALL CASE 🌟 核心修复：在末尾把 ctx 捎上，凑齐 4 个参数
        match CommentCase::case_add_comment_unlike(auth.uid, url.comment_id, url.is_unliked, ctx)
            .await
        {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Unlike Error: {:?}", e);
                AppData::err(5001, "评论不喜欢失败", None)
            }
        }
    }
}

//////// END
