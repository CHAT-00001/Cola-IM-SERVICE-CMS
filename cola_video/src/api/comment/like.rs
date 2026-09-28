// cola_video/src/api/comment/like.rs -- VIDEO - API - 评论 - 点赞互动接口
// 2026/9/29 03:10 Created.

////////

use crate::case::comment::like::CommentLikeCase;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use port::app::ctx::AppContext;

////////

/// # [ADD API] - 视频评论点赞接口
pub struct CommentLikeApi;

impl CommentLikeApi {
    //

    ////////

    /// # 1. [API] - 点赞评论
    /// * `desc`: 解析评论 Command 并调用评论 CASE
    pub async fn like_comment(
        auth: AuthContext,      // 可信会话
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 应用上下文
    ) -> AppData<bool> {
        let body = url
            .body
            .as_ref()
            .and_then(|body| body.get("cmd").or(Some(body)));

        // 🔍 - 快速检查评论 ID
        let comment_id = body
            .and_then(|body| body.get("comment_id").and_then(serde_json::Value::as_i64))
            .filter(|value| *value > 0)
            .unwrap_or(url.comment_id);
        if comment_id <= 0 {
            return AppData::err(4002, "comment_id 不能为空", None);
        }

        // 🔍 - 检查评论状态
        let comment_exists = match ctx.video.comment.check.exists_active(comment_id).await {
            Ok(exists) => exists,
            Err(error) => {
                return AppData::err(5000, format!("评论状态检查失败: {error}"), None);
            }
        };
        if !comment_exists {
            return AppData::err(4004, "评论不存在", None);
        }

        let action_id = body
            .and_then(|body| body.get("action_id").or_else(|| body.get("_id")))
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .or_else(|| url.action_id.clone());
        let Some(action_id) = action_id else {
            return AppData::err(4002, "action_id 不能为空且必须是 UUID v4", None);
        };
        let state = body
            .and_then(|body| {
                body.get("is_liked")
                    .or_else(|| body.get("is_like"))
                    .or_else(|| body.get("state"))
                    .or_else(|| body.get("is"))
            })
            .and_then(serde_json::Value::as_bool)
            .or(url.is)
            .unwrap_or(true);

        match CommentLikeCase::case_set_comment_like(auth.uid, comment_id, state, action_id, ctx)
            .await
        {
            Ok(result) => AppData::ok(result).with_msg("评论点赞操作成功"),
            Err(error) => AppData::err(5000, format!("评论点赞失败: {error}"), None),
        }
    }
}

//////// END
