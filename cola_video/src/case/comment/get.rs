// cola_video/src/case/comment/get.rs -- VIDEO - CASE - 评论 - 获取用例编排
// 2026/9/11 10:01 Created.

////////

use crate::assembler::comment::build_comment_list_response;
use anyhow::Result;
use cola_data::cola_video::vo::comment::CommentListResponse;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 视频评论获取用例
pub struct CommentGetCase;

impl CommentGetCase {
    //

    ////////

    /// # 1. [CASE] - 获取视频的评论
    /// * `desc`: `根据视频 ID 分页获取评论`
    pub async fn case_get(
        uid: i64,        // 操作者 ID
        video_id: i64,   // 视频 ID
        limit: i64,      // 数量
        offset: i64,     // 偏移
        ctx: AppContext, // 应用上下文
    ) -> Result<CommentListResponse> {
        ////////

        // 💡 - 评论信息
        let infos = ctx
            .video
            .comment
            .get
            .get_comment_by_video_id(video_id, limit, offset)
            .await
            .map_err(|error| anyhow::anyhow!("评论列表查询失败: {error}"))?;
        let total = infos.len() as i64;
        info!(
            "[🗣️ COMMENT CASE] - ✅️ 评论列表查询成功: uid={uid}, video_id={video_id}, count={total}"
        );

        ////////

        // 🚧 - 组装响应
        build_comment_list_response(infos, Some(uid), 1, limit, total, &ctx).await
    }


    ////////

    /// # 2. [CASE] - 获取用户的评论
    /// * `desc`: `根据用户 ID 分页获取评论`
    pub async fn case_get_comments_by_user_id(
        uid: i64,        // 操作者 ID
        video_id: i64,   // 视频 ID
        limit: i64,      // 数量
        offset: i64,     // 偏移
        ctx: AppContext, // 应用上下文
    ) -> Result<CommentListResponse> {
        ////////

        // 💡 - 评论信息
        let infos = ctx
            .video
            .comment
            .get
            .get_comment_by_video_id(video_id, limit, offset)
            .await
            .map_err(|error| anyhow::anyhow!("评论列表查询失败: {error}"))?;
        let total = infos.len() as i64;
        info!(
            "[🗣️ COMMENT CASE] - ✅️ 评论列表查询成功: uid={uid}, video_id={video_id}, count={total}"
        );

        ////////

        // 🚧 - 组装响应
        build_comment_list_response(infos, Some(uid), 1, limit, total, &ctx).await
    }
}

//////// END
