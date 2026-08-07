// /manage.rs
//
// 2026/8/5 15:52 Created.

////////

use crate::video::command::report::VideoReportCommand;

/// # [MANAGE PORTS] - 管理
/// * `desc`: `举报管理端口`
#[async_trait::async_trait]
pub trait ReportManagePort: Send + Sync {
    ////////

    /// # 1. [PORT] - 保存
    async fn save_report_record(
        &self,
        uid: i64,
        video_id: i64,
        cmd: VideoReportCommand,
    ) -> anyhow::Result<()>;

    ////////

    ////////

    /// # 2. [PORT] - 获取举报的视频IDs
    async fn get_report_record_ids(
        &self,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(i64, Vec<i64>)>;
}

//////// END
