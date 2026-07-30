// repo_adapter/src/gis/report.rs
// 2026-07-07 12:10

////////

use async_trait::async_trait;
use cola_data::gis::port::report::ReportRepo;
use cola_data::gis::command::report::PoiReportCommand;
use repository::gis::service::report::ReportService;

////////

/// # [REPORT PORT] - 举报 端口 插头
pub struct ReportPortAdapter;

////////

#[async_trait]
impl ReportRepo for ReportPortAdapter {

    ////////

    /// # 1. [PORT] - 保存举报记录
    async fn save_report_record(
        &self,
        _uid: i64,
        _poi_id: i64,
        _cmd: PoiReportCommand,
    ) -> anyhow::Result<()> {
        // TODO: implement with GIS report service
        Ok(())
    }

    ////////

    /// # 2. [PORT] - 获取举报记录的ids
    async fn get_report_record_ids(
        &self,
        _offset: i64,
        _limit: i64,
    ) -> anyhow::Result<(i64, Vec<i64>)> {
        Ok((0, vec![]))
    }
}

//////// END