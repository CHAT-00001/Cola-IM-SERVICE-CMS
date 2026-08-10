// data/src/cola_video/vo/comment.rs
// 🗄 数据 - ▶ 可乐视频 - VO - 评论视图对象
// 2026/8/8 14:05 Updated - 统一使用泛型 ListResponse<T>

////////

use crate::cola_video::info::comment::VideoCommentInfo;
use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;

////////

/// # [VO] - 单评论响应（已废弃，请使用泛型）
#[deprecated(since = "2026-08-08", note = "Use ListResponse<CommentVo> instead")]
#[derive(Debug, Serialize)]
pub struct CommentSingleResponse {
    pub info: VideoCommentInfo, // 吐给前端完美的、组装好的单体 VO
}

// ⚠️ 注意：CommentListResponse 已被移除，现在直接返回 ListResponse<CommentVo>
// 使用方式：
// let list_response = ListResponse {
//     list: vec![comment_vo, ...],
//     page_info: PageInfo { page, qty, has_more },
// };

//////// END
