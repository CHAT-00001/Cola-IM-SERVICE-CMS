// cola_video/src/api/comment/add.rs -- VIDEO - API - 评论 - 发布接口
// 2026/9/11 Created.

////////

use crate::case::comment::add::CommentAddCase;
use crate::utils::CheckId;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::vo::comment::CommentSingleResponse;
use port::app::ctx::AppContext;
use port::cola_video::video::check::VideoCommentAccess;

////////

/// # [ADD API] - 视频评论发布接口
pub struct CommentAddApi;

impl CommentAddApi {
    //

    ////////

    /// # 1. [API] - 发布评论
    /// * `desc`: 解析评论 Command 并调用评论 CASE
    pub async fn add_comment(
        auth: AuthContext,      // 可信会话
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 应用上下文
    ) -> AppData<CommentSingleResponse> {
        // 🔍 - 参数获取
        let cmd_value = url
            .body
            .as_ref()
            .map(|body| body.get("cmd").cloned().unwrap_or_else(|| body.clone()))
            .unwrap_or_default();
        let cmd: CommentCommand = match serde_json::from_value(cmd_value) {
            Ok(value) => value,
            Err(error) => return AppData::err(4002, format!("评论参数解析失败: {error}"), None),
        };

        // 🔍 - 参数检查
        let video_id = if url.id > 0 {
            url.id
        } else if url.video_id > 0 {
            url.video_id
        } else {
            cmd.video_id
        };
        if !CheckId::video_id(video_id) || cmd.content.trim().is_empty() {
            return AppData::err(4002, "id 和 content 不能为空", None);
        }

        // 🔍 - 视频状态检查
        match ctx.video.video.check.check_comment_access(video_id).await {
            Ok(VideoCommentAccess::Allowed) => {}
            Ok(VideoCommentAccess::NotFound) => {
                return AppData::err(4004, "视频不存在", None);
            }
            Ok(VideoCommentAccess::Forbidden) => {
                return AppData::err(4003, "没有评论权限", None);
            }
            Err(error) => {
                return AppData::err(5000, format!("视频状态检查失败: {error}"), None);
            }
        }

        // 💡 - CASE 层
        match CommentAddCase::case_send(auth.uid, video_id, cmd, ctx.clone()).await {
            Ok(response) => AppData::ok(response).with_msg("评论发布成功"),
            Err(error) => {
                let error_text = error.to_string();
                if error_text.contains("评论已经存在") {
                    AppData::err(5001, "评论已经存在", None)
                } else {
                    AppData::err(5000, format!("评论发布失败: {error}"), None)
                }
            }
        }
    }
}

//////// END
