// cola_gis/src/case/add  -- GIS - 用例层 - 评论
// 2026-07-07

////////

use crate::assembler::comment::{build_comment_list_response, build_comment_single_response};
use crate::model::vo::poi_comment::{CommentListResponse, CommentSingleResponse};
use anyhow::Result;
use cola_data::app::ctx::AppContext;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::cola_gis::command::comment::PoiCommentCommand;
use repository::cola_gis::service::poi_comment::PoiCommentService;

////////

/// # [COMMENT CASE] - 评论 用例
pub struct CommentCase;

////////
impl CommentCase {
    ////////

    /// # 1. [CASE] - 发布评论
    pub async fn case_add_comment(
        uid: i64,
        poi_id: i64,
        cmd: PoiCommentCommand,
        ctx: &AppContext,
    ) -> Result<CommentSingleResponse> {
        let visibility = 5;

        let infos =
            PoiCommentService::save_comment_and_update_count(uid, poi_id, cmd, visibility).await?;

        let response = build_comment_single_response(infos, Some(uid)).await?;

        Ok(response)
    }

    ////////

    /// # 2. [CASE] - 浏览兴趣点评论列表
    pub async fn case_get_poi_comments_list(
        uid: i64,
        poi_id: i64,
        query: ApiUrlParamsQuery,
        _ctx: &AppContext,
    ) -> Result<CommentListResponse> {
        let infos =
            PoiCommentService::find_comments_by_poi_id(poi_id, query.offset, query.limit).await?;

        let current_page_total = infos.len() as i64;

        let response =
            build_comment_list_response(infos, Some(uid), query.page.unwrap_or(1), query.qty.unwrap_or(10), current_page_total).await?;

        Ok(response)
    }

    ////////

    /// # 3. [CASE] - 删除一条评论
    pub async fn case_del_one_item(
        uid: i64,
        comment_id: i64,
        ctx: &AppContext,
    ) -> Result<String> {
        ctx.gis.comment
            .del_comment_record(comment_id).await?;
        Ok("单条删除评论成功~".to_string())
    }

    /////////

    /// # 4. [CASE] - 批量删除评论
    pub async fn case_del_all_item(
        comment_ids: Vec<i64>,
        ctx: &AppContext,
    ) -> Result<String> {
        ctx.gis.comment
            .del_comments_record(comment_ids).await?;
        Ok("批量删除评论成功~".to_string())
    }

    ////////
}

////// END