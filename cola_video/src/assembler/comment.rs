// cola_video/src/assembler/comment.rs  -- VIDEO - 组装 - 评论响应体组装
// 2026/06/05 09:50 by wx: cestbon10080

////////

use anyhow::{anyhow, Result};
use cola_data::app::page::PageInfo;
use cola_data::user::info::user::UserInfo;
use cola_data::video::entity::comment::CommentEntity;
use crate::model::info::comment::CommentInfo;
use crate::model::vo::comment::{CommentListResponse, CommentSingleResponse, CommentVo};
use repo::user::service::user::UserService; // 🚀 使用你定义的静态 UserService

////////

/// # [ASSEMBLER] - 组装单评论响应
pub async fn build_comment_single_response(
    entity: CommentEntity,
    current_uid: Option<i64>,
) -> Result<CommentSingleResponse> {

    // 1. 静态调用：获取作者信息 (内部处理了 None 情况的 default 兜底)
    let author = UserService::find_user_info_by_id(entity.user_id).await?;

    // 2. 组装 VO
    let comment_info = CommentInfo::from_entity(entity);
    let comment_vo = CommentVo::from_info(
        comment_info,
        current_uid.unwrap_or(0),
        author, // 🚀 补全 Author 信息
        false,  // is_liked
        false,  // is_disliked
    );

    Ok(CommentSingleResponse { info: comment_vo })
}

////////

/// # [ASSEMBLER] - 组装多评论列表
pub async fn build_comment_list_response(
    entities: Vec<CommentEntity>,
    current_uid: Option<i64>,
    page: i64,
    qty: i64,
    total: i64,
) -> Result<CommentListResponse> {

    // 1. 静态调用：批量获取用户信息 (UserService 保证了有多少 id 就吐多少 Info，自动填充默认值)
    let author_ids: Vec<i64> = entities.iter().map(|e| e.user_id).collect();
    let authors_map = UserService::find_user_info_by_uids(&author_ids).await?;

    // 2. 迭代组装
    let comments: Vec<CommentVo> = entities.into_iter().map(|entity| {
        // 🚀 补全：从 map 中拿取 author，没找到则默认 (虽然 UserService 已经做过兜底，但这里写 defensive code 更稳)
        let author = authors_map.get(&entity.user_id).cloned().unwrap_or_default();

        let comment_info = CommentInfo::from_entity(entity);

        CommentVo::from_info(
            comment_info,
            current_uid.unwrap_or(0),
            author, // 🚀 传入组装
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