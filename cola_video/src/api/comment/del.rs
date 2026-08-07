// cola_video/src/api/comment/del.rs
// 视频 - 应用层 - 评论 - 删除
// 2026/8/4 18:42 Created.

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

/// # [DEL API]
/// * `desc`: `评论删除接口`
pub struct VideoCommentDelApi;

impl VideoCommentDelApi {
    // 💡

    ////////

    /// # 1. [API HANDLER] - 单条删除
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
