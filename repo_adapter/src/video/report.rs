// repo_adapter/src/new/report.rs
// 2026-06-12

////////

use async_trait::async_trait;
use cola_data::video::port::report::ReportRepo;
use cola_data::video::command::report::VideoReportCommand;
use repository::video::service::report::ReportService;

////////

/// # [REPORT PORT] - 举报 端口 插头
pub struct ReportPortAdapter;

////////

#[async_trait]
impl ReportRepo for ReportPortAdapter {

    ////////

    /// 1. [PORT] - 保存举报记录 + 更新举报数量
    async fn save_report_record(
        &self,
        uid: i64,
        _video_id: i64,
        cmd: VideoReportCommand,
    ) -> anyhow::Result<()> {
        ReportService::save_comment_and_update_count(uid, cmd).await?;
        Ok(())
    }

    ////////

    /// # 2. [PORT] - 获取举报记录的ids
    async fn get_report_record_ids(
        &self,
        _offset: i64,
        _limit: i64,
    ) -> anyhow::Result<(i64, Vec<i64>)> {
        // TODO: implement with real repository
        Ok((0, vec![]))
    }
}

//////// END