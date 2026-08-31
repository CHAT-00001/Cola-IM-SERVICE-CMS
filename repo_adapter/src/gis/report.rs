// repo_adapter/src/cola_gis/report.rs
// 🔌 适配器 - 可乐GIS - POI - 举报
// 2026-07-07 12:10

////////

use async_trait::async_trait;
use cola_data::cola_gis::command::report::PoiReportCommand;
use port::cola_gis::report::ReportRepo;
use repository::cola_gis::service::report::ReportService;

////////

/// # [REPORT ADAPTER] - 举报 端口 插头
pub struct ReportPortAdapter;

////////

#[async_trait]
impl ReportRepo for ReportPortAdapter {
    //

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
