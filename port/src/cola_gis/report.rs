// cola_data/src/cola_gis/port/report.rs  --
// ⏩️ 端口 - 可乐GIS - POI -  举报
// 2026/7/7

////////

use cola_data::cola_gis::command::report::PoiReportCommand;

////////

/// # [SERVICE] - 举报
#[async_trait::async_trait]
pub trait ReportRepo: Send + Sync {
    ////////

    /// # 1. [PORT] - 保存举报记录
    async fn save_report_record(
        &self,
        uid: i64,
        poi_id: i64,
        cmd: PoiReportCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 获取举报记录的IDs
    async fn get_report_record_ids(
        &self,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(i64, Vec<i64>)>;
}

////// END
