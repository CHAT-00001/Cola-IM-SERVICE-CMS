// service/src/cola_video/report/list.rs
// 👤 服务 - ▶ 可乐视频 - 举报 - 列表
// 2026/8/9 01:45 Created.

////////

use anyhow::Result;
use cola_data::cola_video::command::report::VideoReportCommand;
use cola_data::cola_video::entity::comment::VideoCommentEntity;
use tracing::log;

////////

/// # [LIST SERVICE] - 列表
/// * `desc`: `▶ 可乐视频 - 视频举报列表服务`
pub struct VideoReportListService;

// 构造实现
impl VideoReportListService {
    //

    ////////

    /// # 1. [SERVICE] - 获取举报相关数据/列表
    pub async fn save_comment_and_update_count(
        uid: i64,
        cmd: VideoReportCommand,
    ) -> Result<Vec<VideoCommentEntity>> {
        let _ = (uid, cmd);
        // TODO: 替换为你底层的 Repo 真实查询
        let mock_entity = VideoCommentEntity::default();
        let saved_list = vec![mock_entity];

        Ok(saved_list)
    }

    ////////

    /// # 2. [SERVICE] - 检查视频状态
    pub async fn check_video_state(video_id: i64) -> Result<i32> {
        let _ = video_id;
        let code = 4001;
        Ok(code)
    }

    ////////

    /// # 3. [SERVICE] - 检查用户状态
    pub async fn check_user_state(user_id: i64) -> Result<i32> {
        let _ = user_id;
        let code = 4001;
        Ok(code)
    }

    ////////
}

//////// END