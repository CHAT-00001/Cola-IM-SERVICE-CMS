// video/api/report/manage.rs
// 视频 - api - 举报 - 管理
// 2026/8/4 18:53 Created.

////////

use crate::case::report::ReportCase;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::auth::info::auth::AuthContext;
use cola_data::video::command::report::VideoReportCommand;

////////

/// # [API HANDLER] - 举报
/// * `desc`: `视频举报管理接口`
pub struct VideoReportManageApi;

// 构造函数
impl VideoReportManageApi {
    //

    ////////

    /// # 1. [USE CASE] -  添加
    pub async fn api_get_new_report_list(
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

    /// # 2. [USE CASE] - 浏览
    /// * `desc`: `浏览举报记录的视频`
    pub async fn api_get_report_list(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<String> {

        let uid = auth.uid;

        // Call Case:
        match ReportCase::case_get_report_video(uid, url, ctx).await {
            Ok(_) => AppData::ok("获取成功".to_string()).with_msg("✅️ 获取被举报的内容列表成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("❌️ 获取被举报的内容列表失败: {:?}", e),
                None,
            ),
        }
    }

    ////////

    /// # 4. [USE CASE] - 处理
    /// * `desc`: `处理举报动作`
    pub async fn api_process_report_action(
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

//////// END