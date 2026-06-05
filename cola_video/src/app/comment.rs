// cola_video/src/video/app/comment.rs  -- VIDEO - 应用层 - 评论
// 2026/4/24 16:04

////////

use crate::biz;
use cola_data::app::data::AppData;
use cola_data::video::command::comment::CommentCommand;
use serde::{Deserialize, Serialize};
////////

/// ## 互动请求参数
#[derive(Deserialize, Debug)]
pub struct InteractRequest {
    pub uid: i64,         // 操作人 ID
    pub target_id: i64,   // 目标对象 ID (视频ID)
    pub action_type: i16, // 动作类型/举报类型
}

// 内部辅助函数：统一处理 Biz 到 AppData 的转换
async fn wrap_result(res: anyhow::Result<()>, err_code: i32) -> AppData<()> {
    match res {
        Ok(_) => AppData::ok(()),
        Err(e) => AppData::err(err_code, e.to_string(), None),
    }
}

// --- UseCase 业务编排层 ---
pub struct VideoCommentCase;

impl VideoCommentCase {
    /// # 1. [CASE] - 发布评论
    pub async fn case_add_comment(
        uid: i64,            // 操作者 ID
        cmd: CommentCommand, // 评论创建命令
    ) -> AppData<()> {
        wrap_result(biz::like::add_comment_logic(uid, cmd).await, 5001).await
    }

    /// # 2. 获取评论列表
    pub async fn case_home_comment(uid: i64, comment_id: i64) -> AppData<()> {
        wrap_result(biz::like::remove_like_logic(uid, comment_id).await, 5002).await
    }

    /// # 3. 修改评论
    pub async fn case_change_comment(uid: i64, comment_id: i64) -> AppData<()> {
        wrap_result(biz::like::add_collect_logic(uid, comment_id).await, 5003).await
    }

    /// # 4. 删除评论
    pub async fn case_delete_comment(uid: i64, comment_id: i64) -> AppData<()> {
        wrap_result(biz::like::remove_collect_logic(uid, comment_id).await, 5004).await
    }

    /// # 5. 评论点赞
    pub async fn case_add_like(comment_id: i64) -> AppData<()> {
        wrap_result(
            biz::like::increment_view_logic(comment_id, port).await,
            5005,
        )
        .await
    }

    /// # 6. 评论取消点赞
    pub async fn case_del_like(uid: i64, comment_id: i64) -> AppData<()> {
        wrap_result(biz::like::mark_done_logic(uid, comment_id).await, 5006).await
    }

    /// # 7. 踩评论
    pub async fn case_add_step(comment_id: i64) -> AppData<()> {
        wrap_result(biz::like::set_hot_logic(comment_id, port).await, 5007).await
    }

    /// # 8. 取消踩
    pub async fn case_del_step(req: InteractRequest) -> AppData<()> {
        wrap_result(
            biz::like::report_logic(req.uid, req.target_id, req.action_type, port).await,
            5008,
        )
        .await
    }

    /// # 9. 举报评论
    pub async fn case_report_comment(req: InteractRequest) -> AppData<()> {
        wrap_result(
            biz::like::report_logic(req.uid, req.target_id, req.action_type, port).await,
            5009,
        )
        .await
    }
}

//////// END
