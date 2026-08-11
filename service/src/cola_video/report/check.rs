// service/src/cola_video/report/check.rs
// 👤 服务 - ▶ 可乐视频 - 举报 - 检查
// 2026/8/9 01:47 Created.

////////

use anyhow::Result;
use cola_data::cola_video::command::report::VideoReportCommand;
use cola_data::cola_video::entity::comment::VideoCommentEntity;
use tracing::log;

////////

/// # [CHECK SERVICE] - 列表
/// * `desc`: `▶ 可乐视频 - 视频举报列表服务`
pub struct VideoReportCheckService;

// 构造实现
impl VideoReportCheckService {
    //

    ////////

    /// # 1. [SERVICE] - 检查健康
    pub async fn check_health(video_id: i64) -> Result<i32> {
        let _ = video_id;
        let code = 4001;
        Ok(code)
    }

    ////////

    /// # 2. [SERVICE] - 检查状态
    pub async fn check_state(video_id: i64) -> Result<i32> {
        let _ = video_id;
        let code = 4001;
        Ok(code)
    }

    ////////
}

//////// END