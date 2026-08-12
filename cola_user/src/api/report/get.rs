// core_user/src/api/report/get.rs
// 可乐用户 - api - report - 获取
// 2026/8/2 22:40 Created.

////////

use crate::case::report::list::UserReportListCase;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::info::auth::AuthContext;
use cola_data::cola_video::info::video::VideoListResponse;
use port::app::ctx::AppContext;

////////

/// # [REPORT GET API] - 举报列表 接口
pub struct ReportGetApi;

impl ReportGetApi {
    //

    ////////

    /// # 1. [API HANDLER] - 我的举报记录
    pub async fn api_get_my_list(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match UserReportListCase::case_get_my_report_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Get My Reports Error: {:?}", e);
                AppData::err(5001, "❌️ 获取我的举报列表失败", None)
            }
        }
    }

    ////////

    /// # 2. [API HANDLER] - 最新举报记录
    pub async fn api_get_new_list(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match UserReportListCase::case_get_new_report_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Get New Reports Error: {:?}", e);
                AppData::err(5001, "获取最新举报记录失败", None)
            }
        }
    }

    ////////

    /// # 3. [API HANDLER] - 处理过的举报记录
    pub async fn api_get_processed_list(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match UserReportListCase::case_get_processed_report_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Get Processed Reports Error: {:?}", e);
                AppData::err(5001, "获取处理过的举报记录失败", None)
            }
        }
    }

    ////////

    /// # 4. [API HANDLER] - 违规类型列表
    pub async fn api_get_violation_type_list(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match UserReportListCase::case_get_violation_type_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Get Violation Types Error: {:?}", e);
                AppData::err(5006, format!("获取违规类型失败: {}", e), None)
            }
        }
    }

    ////////

    /// # 5. [API HANDLER] - 举报分类列表
    pub async fn api_get_report_category_list(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match UserReportListCase::case_get_report_category_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Get Report Categories Error: {:?}", e);
                AppData::err(5006, format!("获取举报分类失败: {}", e), None)
            }
        }
    }

    ////////

    /// # 6. [API HANDLER] - 处理结果类型列表
    pub async fn api_get_result_type_list(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match UserReportListCase::case_get_result_type_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Get Result Types Error: {:?}", e);
                AppData::err(5001, "获取处理结果类型失败", None)
            }
        }
    }
}

//////// END
