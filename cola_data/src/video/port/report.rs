// cola_data/src/video/port/report.rs  -- 数据中心 - VIDEO - port - 视频 举报
// 2026/6/10 08:28

////////

use crate::video::command::report::VideoReportCommand;

/// # [SERVICE] - 举报
#[async_trait::async_trait]
pub trait ReportRepo: Send + Sync {

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
