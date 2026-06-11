// cola_video/src/assembler/comment.rs  -- 组装 - 评论响应体
// 2026/06/05 09:50 修改成只吃infos

////////

use anyhow::{anyhow, Result};
use cola_data::app::page::PageInfo;
use cola_data::user::info::user::UserInfo;
use cola_data::video::info::comment::CommentInfo; // 🌟 物理 Entity 可以砍了，全线拥抱 Info
use crate::model::vo::comment::{CommentListResponse, CommentSingleResponse, CommentVo};
use repo::user::service::user::UserService;

////////

/// # [ASSEMBLER] - 组装单评论响应
pub async fn build_comment_single_response(
    comment_info: CommentInfo, // 🌟 听哥们的，改成吃 Info
    current_uid: Option<i64>,
) -> Result<CommentSingleResponse> {

    // 1. 静态调用：获取作者信息
    let author = UserService::get_user_info_by_id(comment_info.user_id).await?;

    // 2. 组装 VO (不再需要从 entity 转换，直接原地起飞)
    let comment_vo = CommentVo::from_info(
        comment_info,
        current_uid.unwrap_or(0),
        false,  // is_liked
        false,  // is_disliked
    );

    Ok(CommentSingleResponse { info: comment_vo })
}

////////

/// # [ASSEMBLER] - 组装多评论列表
pub async fn build_comment_list_response(
    infos: Vec<CommentInfo>, // 🌟 同步升级，多列表组装也全部改吃 infos
    current_uid: Option<i64>,
    page: i64,
    qty: i64,
    total: i64,
) -> Result<CommentListResponse> {

    // 1. 静态调用：从 infos 中提取 user_id 批量获取用户信息
    let author_ids: Vec<i64> = infos.iter().map(|info| info.user_id).collect();
    let authors_map = UserService::get_user_info_by_ids(&author_ids).await?;

    // 2. 迭代组装
    let comments: Vec<CommentVo> = infos.into_iter().map(|comment_info| {
        // 🚀 从 map 中拿取 author，没找到则默认兜底
        let author = authors_map.get(&comment_info.user_id).cloned().unwrap_or_default();

        CommentVo::from_info(
            comment_info,
            current_uid.unwrap_or(0),
            false,
            false,
        )
    }).collect();

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