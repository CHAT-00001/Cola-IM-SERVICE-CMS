// cola_video/src/api/video/get.rs -- VIDEO - API - 视频内容 - 获取接口
// 2026/9/11 20:10 Created.

////////

use crate::case::video::get::VideoContentGetCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_data::cola_video::info::video::VideoSingleResponse;
use port::app::ctx::AppContext;

////////

/// # [GET API] - 视频内容获取接口
pub struct VideoContentGetApi;

impl VideoContentGetApi {
    //

    ////////

    /// # 1. [API] - 获取视频详情
    /// * `desc`: 根据视频 ID 分页获取评论
    pub async fn get_video(
        auth: AuthContext,      // 可信会话
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 应用上下文
    ) -> AppData<VideoSingleResponse> {
        // 初步检查视频ID是否存在
        let video_id = if url.id > 0 { url.id } else { url.video_id };
        if video_id <= 0 {
            return AppData::err(4002, "id 不能为空", None);
        }

        // Call CASE ..
        match VideoContentGetCase::case_get_detail(auth.uid, video_id, ctx.clone()).await {
            Ok(response) => AppData::ok(response).with_msg("视频详情获取成功"),
            Err(error_value) => AppData::err(
                error::INTERNAL_ERROR,
                format!("视频详情获取失败: {error_value}"),
                None,
            ),
        }
    }
}

//////// END
