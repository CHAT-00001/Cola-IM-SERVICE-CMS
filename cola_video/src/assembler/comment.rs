// cola_video/src/assembler/add  -- VIDEO - 组装 - 评论响应体
// 2026/06/05 09:50

////////

use crate::model::vo::comment::{CommentListResponse, CommentSingleResponse, CommentVo};
use anyhow::Result;
use cola_data::app::page::PageInfo;
use cola_data::cola_video::info::comment::VideoCommentInfo; // 🌟 物理 Entity 可以砍了，全线拥抱 Info
////////

/// # [ASSEMBLER] - 组装单评论响应
pub async fn build_comment_single_response(
    comment_info: VideoCommentInfo, // 🌟 听哥们的，改成吃 Info
    current_uid: Option<i64>,
) -> Result<CommentSingleResponse> {
    // 1. 组装 VO
    let comment_vo = CommentVo::from_info(
        comment_info,
        current_uid.unwrap_or(0),
        false, // is_liked
        false, // is_disliked
    );

    Ok(CommentSingleResponse { info: comment_vo })
}

////////

/// # [ASSEMBLER] - 组装多评论列表
pub async fn build_comment_list_response(
    infos: Vec<VideoCommentInfo>, // 🌟 同步升级，多列表组装也全部改吃 infos
    current_uid: Option<i64>,
    page: i64,
    qty: i64,
    total: i64,
) -> Result<CommentListResponse> {
    // 1. 迭代组装
    let comments: Vec<CommentVo> = infos
        .into_iter()
        .map(|comment_info| CommentVo::from_info(comment_info, current_uid.unwrap_or(0), false, false))
        .collect();

    // 3. 计算分页
    Ok(CommentListResponse {
        comments,
        page_info: PageInfo {
            page,
            qty,
            has_more: (page * qty) < total,
        },
    })
}

//////// END
