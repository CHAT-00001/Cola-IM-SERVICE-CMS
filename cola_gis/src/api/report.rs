// cola_gis/src/api/report.rs
// 可乐GIS - 接口层 - POI 举报
// 2026/6/10 19:13

////////

use crate::case::report::PoiReportCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::info::auth::AuthContext;
use cola_data::cola_gis::command::report::PoiReportCommand;
use port::app::ctx::AppContext;

////////

/// # [API HANDLER] - 兴趣点 举报 接口
pub struct PoiReportApi;

// 构造实现
impl PoiReportApi {
    //

    ////////

    /// # 1. [CASE] -  添加
    pub async fn handler_add_report(
        auth: &AuthContext,
        url: ApiGatewayRequest,
        cmd: PoiReportCommand,
        ctx: &AppContext,
    ) -> AppData<String> {
        let uid = auth.uid;

        match PoiReportCase::case_add_report(uid, url, cmd, ctx).await {
            Ok(_) => AppData::ok("举报成功".to_string()).with_msg("举报成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("举报失败: {:?}", e), None),
        }
    }

    ////////

    /// # 2. [CASE] - 浏览举报记录的兴趣点
    pub async fn handler_get_video_list(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<String> {
        let uid = auth.uid;

        // Call Case:
        match PoiReportCase::case_get_report_poi(uid, url, ctx).await {
            Ok(_) => AppData::ok("获取成功".to_string()).with_msg("获取被举报的兴趣点列表成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("获取被举报的兴趣点列表失败: {:?}", e),
                None,
            ),
        }
    }

    ////////
}

////////
