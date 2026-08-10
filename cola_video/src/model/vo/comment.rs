// cola_video/src/model/vo/comment.rs  -- VO - 评论视图对象
// 2026/8/8 14:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::app::page::PageInfo;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct CommentVo {
    pub id: i64,                    // 评论 ID
    pub user_id: i64,               // 作者 ID
    pub video_id: i64,              // 关联视频 ID
    pub content: String,            // 评论内容
    pub create_time: i64,           // 创建时间
    pub like_count: i32,            // 点赞数
    pub reply_count: i32,           // 回复数
    pub is_liked: bool,             // 当前用户是否点赞
    pub author: UserInfo,           // 作者信息
}

impl Default for CommentVo {
    fn default() -> Self {
        Self {
            id: 0,
            user_id: 0,
            video_id: 0,
            content: String::new(),
            create_time: 0,
            like_count: 0,
            reply_count: 0,
            is_liked: false,
            author: UserInfo::default(),
        }
    }
}

impl CommentVo {
    /// 从已有的 CommentInfo 组装成最终的 VO 对象
    pub fn from_info(
        info: VideoCommentInfo,
        current_uid: i64,
        is_liked: bool,
        author: UserInfo,
    ) -> Self {
        Self {
            id: info.id,
            user_id: info.user_id,
            video_id: info.video_id,
            content: info.content,
            create_time: info.create_time,
            like_count: info.like_count,
            reply_count: info.reply_count,
            is_liked: is_liked,
            author,
        }
    }
}

//////// END
