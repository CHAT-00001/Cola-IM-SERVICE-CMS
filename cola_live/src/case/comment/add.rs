// cola_live/src/case/file/add.rs
// LIVE - 用例层 - 评论 - 发布
// 2026/8/12 04:51 Created.

////////

use crate::assembler::comment::build_comment_single_response;
use crate::model::vo::comment::CommentSingleResponse;
use anyhow::Result;
use cola_data::cola_video::command::comment::CommentCommand;
use port::app::ctx::AppContext;
use service::cola_video::comment::add::VideoCommentAddService;
use std::collections::HashMap;
use tracing::info;

////////

/// # [ADD CASE] - 评论 发布
/// * `DESC`: 评论发布用例编排
pub struct LiveCommentAddCase;

////////
impl LiveCommentAddCase {
    //

    ////////

    /// # 1. [CASE] - 发布
    /// * `描述`: 用户UGC评论应用编排
    pub async fn case_add_comment(
        uid: i64,
        video_id: i64,
        mut cmd: CommentCommand, // 👈 1. 改为 mut 以便修改内部字段
        ctx: &AppContext,
    ) -> Result<CommentSingleResponse> {
        info!("Case: 开始处理发布评论用例, uid: {}, video_id: {}", uid, video_id);

        // 2. 将路由路径中的 video_id 赋予 Command，确保仓储层能正确读取
        cmd.video_id = video_id;

        // 3. 检查评论内容风控等级（后续可在此处调用风控 Service）
        let key = cmd.content.to_string();
        let visibility = 5;

        // 4. 调用 SERVICE 层（Service层内部会去校验父级评论是否存在并完成入库）
        let infos = VideoCommentAddService::create_comment(uid, visibility, cmd).await?;

        // 5. 组装成 VO (View Object)
        let response = build_comment_single_response(infos, Some(uid)).await?;

        info!("Case: 评论发布流程完成");
        // 6. 返回成功给 API 层响应
        Ok(response)
    }
}

//////// END