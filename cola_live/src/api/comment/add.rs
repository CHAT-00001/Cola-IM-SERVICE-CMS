// cola_live/src/api/comment/add.rs
// LIVE - api - 评论
// 2026/8/12 05:50 Created.

////////

use crate::case::comment::add::LiveCommentAddCase;
use crate::model::vo::comment::CommentSingleResponse;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::info::auth::AuthContext;
use cola_data::cola_video::command::comment::CommentCommand;
use port::app::ctx::AppContext;

////////

/// # [ADD API] - 发布
pub struct LiveCommentAddApi;

impl LiveCommentAddApi {
    //

    ////////

    /// # 1. [API HANDLER] - 发布
    pub async fn api_add_comment(
        auth: AuthContext, // 验证中心
        url: ApiGatewayRequest,
        cmd: CommentCommand, // 评论命令
        ctx: &AppContext,    // 🌟 核心修复：把业务上下文注入进来
    ) -> AppData<CommentSingleResponse> {
        // 1. 检查用户状态

        // 2. 检查评论权限
        let video_id = url.video_id;

        // 3. 调用CASE (补充传入 ctx)
        match LiveCommentAddCase::case_add_comment(auth.uid, video_id, cmd, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("New Videos Error: {:?}", e);
                AppData::err(5001, "发布评论失败", None)
            }
        }
    }
}

//////// END
