// cola_video/src/case/video/get.rs -- VIDEO - CASE - 视频内容 - 获取用例
// 2026/9/11 02:02 Created.

////////

use crate::assembler::comment::build_comment_list_response;
use crate::assembler::video::build_video_single_response_with_cdn;
use crate::case::storage::resolve_video_cdn_domain;
use crate::model::vo::comment::CommentListResponse;
use anyhow::Result;
use cola_data::cola_video::info::video::VideoSingleResponse;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [GET CASE] - 视频内容获取用例
pub struct VideoContentGetCase;

impl VideoContentGetCase {
    //

    ////////

    /// # 1. [CASE] - 获取视频内容
    /// * `desc`: 根据视频 ID 分页获取评论
    pub async fn case_get_detail(
        uid: i64,        // 操作者 ID
        video_id: i64,   // 视频 ID
        ctx: AppContext, // 应用上下文
    ) -> Result<VideoSingleResponse> {
        // Call CTX .. INFO
        let info = ctx
            .video
            .video
            .get
            .get_video_info_by_id(uid, video_id)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 CASE] ❌️视频信息查询失败: {error}"))?;
        info!("[🗣️ CASE] - ✅️ 视频信息查询成功: uid={uid}, video_id={video_id}");

        let cdn_domain = resolve_video_cdn_domain(&ctx, "short-video").await?;
        build_video_single_response_with_cdn(info, Some(uid), &cdn_domain).await
    }
}

//////// END
