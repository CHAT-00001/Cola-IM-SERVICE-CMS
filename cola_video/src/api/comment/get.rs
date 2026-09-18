// cola_video/src/api/comment/get.rs -- VIDEO - API - 评论 - 获取接口
// 2026/9/11 17:10 Created.

////////

use crate::case::comment::get::CommentGetCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_data::cola_video::vo::comment::CommentListResponse;
use port::app::ctx::AppContext;

////////

/// # [GET API] - 视频评论获取接口
pub struct CommentGetApi;

impl CommentGetApi {
    //

    ////////

    /// # 1. [API] - 获取评论
    /// * `desc`: 根据视频 ID 分页获取评论
    pub async fn get_comment(
        auth: AuthContext,      // 可信会话
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 应用上下文
    ) -> AppData<CommentListResponse> {
        let video_id = if url.id > 0 { url.id } else { url.video_id };
        if video_id <= 0 {
            return AppData::err(4002, "id 不能为空", None);
        }

        let limit = url.qty.unwrap_or(20).clamp(1, 50);
        match CommentGetCase::case_get(auth.uid, video_id, limit, url.offset, ctx.clone()).await {
            Ok(response) => AppData::ok(response).with_msg("评论列表获取成功"),
            Err(error_value) => AppData::err(
                error::INTERNAL_ERROR,
                format!("评论列表获取失败: {error_value}"),
                None,
            ),
        }
    }
}

//////// END
