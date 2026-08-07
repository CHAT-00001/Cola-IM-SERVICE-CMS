// cola_video/api/report/get.rs
// 
// 2026/8/4 18:53 Created.

////////


// /report.rs  -- 接口层 举报
// 2026/6/10 19:13

////////


use crate::case::report::ReportCase;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::cola_auth::info::auth::AuthContext;
use cola_data::cola_video::command::report::VideoReportCommand;

////////

/// # [API HANDLER] - 举报
/// * `desc`: `视频举报接口`
pub struct VideoReportGetApi;

// 构造函数
impl VideoReportGetApi {
    //

    ////////

    /// # 1. [API HANDLER] -  添加
    pub async fn handler_add_report(
        auth: &AuthContext,
        url: ApiGatewayRequest,
        cmd: VideoReportCommand,
        ctx: &AppContext,
    ) -> AppData<String> {
        let uid = auth.uid;

        match ReportCase::case_add_report(uid, url, cmd, ctx).await {
            Ok(_) => AppData::ok("举报成功".to_string()).with_msg("举报成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("举报失败: {:?}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 浏览举报记录的视频
    pub async fn handler_get_video_list(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<String> {

        let uid = auth.uid;

        // Call Case:
        match ReportCase::case_get_report_video(uid, url, ctx).await {
            Ok(_) => AppData::ok("获取成功".to_string()).with_msg("获取被举报的视频列表成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("获取被举报的视频列表失败: {:?}", e),
                None,
            ),
        }
    }

    ////////
}

////////
