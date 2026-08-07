// report/add.rs
// 视频 - port - 举报 - 发布
// 2026/8/5 15:51 Created.

////////

use crate::video::command::report::VideoReportCommand;

////////

/// # [ADD PORTS] - 发布
/// * `desc`: `用户发布举报端口`
#[async_trait::async_trait]
pub trait ReportAddPort: Send + Sync {
    //

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