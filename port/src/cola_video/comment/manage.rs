// port/src/cola_video/comment/manage.rs -- PORT - VIDEO - 评论 - 管理服务
// 2026/8/5 15:23 Created.

////////

use cola_data::cola_video::info::comment::CommentInfo;

////////

/// # [MANAGE PORTS] - 评论管理服务端口
/// * `desc`: `VIDEO - Comment Manage Service Ports.`
#[async_trait::async_trait]
pub trait VideoCommentManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 管理员列表
    /// * `desc`: `获取管理员列表`
    /// * `condition`: `⚠️ WARNING 仅限管理员 / 运营人员`
    async fn admin_list(
        &self,
        uid: i64,                // 操作者 ID
        user_id: Option<i64>,    // 用户 ID
        video_id: Option<i64>,   // 视频 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<CommentInfo>, u64)>;
}

//////// END
