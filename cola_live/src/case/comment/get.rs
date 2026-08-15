// cola_live/src/case/file/get.rs
// LIVE - 用例层 - 评论 - GET
// 2026/8/12 04:51 Created.

////////

use crate::assembler::comment::build_comment_list_response;
use crate::model::vo::comment::CommentListResponse;
use anyhow::Result;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use port::app::ctx::AppContext;
use service::cola_video::comment::get::CommentGetService;

////////

/// # [GET CASE] - 评论 获取
/// * `desc`: 评论查看用例编排
pub struct CommentGetCase;

////////
impl CommentGetCase {
    //

    ////////

    /// # 1. [CASE] - 浏览视频评论列表
    pub async fn case_get_video_comments_list(
        uid: i64,
        video_id: i64,
        query: ApiGatewayRequest,
        _ctx: &AppContext,
    ) -> Result<CommentListResponse> {
        // 1. ✅ 核心修复：干掉元组解构，直接用单变量接住 Vec<CommentInfo>
        let infos =
            CommentGetService::get_comments_by_video_id(video_id, query.offset, query.limit)
                .await?;

        // 2. 拿到当前页的局部数量，用来给下层组装的分页算总量兜底
        let current_page_total = infos.len() as i64;

        // 3. 直接调用组装器
        let response = build_comment_list_response(
            infos,
            Some(uid),
            query.page.unwrap_or(1),
            query.qty.unwrap_or(10),
            current_page_total,
        )
        .await?;

        Ok(response)
    }

    ////////

    /// # 2. [CASE] - 浏览用户发布评论列表
    /// * `描述`: 获取我用户的评论列表
    pub async fn case_get_user_comments_list(
        uid: i64,                 // 操作者 ID
        query: ApiGatewayRequest, // url查询参数
        ctx: &AppContext,
    ) -> Result<CommentListResponse> {
        // Call Service
        let infos =
            CommentGetService::get_comments_by_user_id(uid, query.limit, query.offset).await?;

        // 2. 拿到当前页的局部数量，用来给下层组装的分页算总量兜底
        let current_page_total = infos.len() as i64;

        let response = build_comment_list_response(
            infos,
            Some(uid),
            query.page.unwrap_or(1),
            query.qty.unwrap_or(10),
            current_page_total,
        )
        .await?;

        Ok(response)
    }
}

//////// END
