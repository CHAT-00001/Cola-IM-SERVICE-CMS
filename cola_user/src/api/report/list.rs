// user/api/report/list.rs
// 可乐用户 - api - 举报 - 列表
// 2026/8/5 20:11 Created.

////////

use crate::case;
use crate::case::report::list::UserReportListCase;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::auth::info::auth::AuthContext;
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_data::cola_video::info::video::VideoListResponse;
use port::app::ctx::AppContext;

////////

/// # [USER REPORT LIST API] - 用户举报列表接口
pub struct UserReportListApi;

impl UserReportListApi {
    //

    ////////

    /// # 1. [API HANDLER] - 我的
    /// * `desc`: `用户获取自己的举报记录`
    /// * `condition`: `需要登录`
    pub async fn api_my(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match UserReportListCase::case_get_my_report_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get My Reports Success!");
                AppData::ok(resp)
            }

            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Get My Reports Error: {:?}", e);
                AppData::err(5001, "获取我的举报记录失败", None)
            }
        }
    }

    ////////

    /// # 2. [API HANDLER] - 最新
    /// * `desc`: `管理员获取最新的举报记录`
    /// * `condition`: `需要登录` + `内容审核员角色`
    pub async fn api_get_new(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match UserReportListCase::case_get_new_report_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get New Reports Success!");
                AppData::ok(resp)
            }

            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Get New Reports Error: {:?}", e);
                AppData::err(5001, "获取最新举报记录失败", None)
            }
        }
    }

    ////////

    /// # 3. [API HANDLER] - 处理过的记录
    /// * `desc`: `管理员获取自己处理过的记录`
    /// * `condition`: `需要登录` + `审核员身份`
    pub async fn api_get_processed(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match UserReportListCase::case_get_processed_report_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get Processed Reports Success!");
                AppData::ok(resp)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Get Processed Reports Error: {:?}", e);
                AppData::err(5001, "获取处理过的记录失败", None)
            }
        }
    }

    ////////

    /// # 4. [API HANDLER] - 类型
    /// * `desc`: `前端获取违规类型`
    /// * `condition`: ``
    pub async fn api_get_violation_type(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;
        let category_id = url.category_id;

        if category_id <= 0 {
            return AppData::err(4002, "参数错误：非法的 category_id", None);
        }

        match UserReportListCase::case_get_violation_type_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get Violation Types Success!");
                AppData::ok(resp)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Get Violation Types Error: {:?}", e);
                AppData::err(5006, format!("获取违规类型失败: {}", e), None)
            }
        }
    }

    /// # 5. [API HANDLER] - 分类
    /// * `desc`: `前台获取`
    /// * `condition`: `需要登录`
    pub async fn api_get_category(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;
        let channel_id = url.category_id;

        if channel_id <= 0 {
            return AppData::err(4002, "参数错误：非法的 channel_id", None);
        }

        match UserReportListCase::case_get_report_category_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get Report Categories Success!");
                AppData::ok(resp)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Get Report Categories Error: {:?}", e);
                AppData::err(5006, format!("获取举报分类失败: {}", e), None)
            }
        }
    }

    ////////

    /// # 6. [API HANDLER] - 精选
    /// * `desc`: `管理员获取处理结果类型`
    /// * `condition`: `需要登录` + `审核人员身份`
    pub async fn api_get_result_type(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match UserReportListCase::case_get_result_type_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get Result Types Success!");
                AppData::ok(resp)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Get Result Types Error: {:?}", e);
                AppData::err(5001, "获取处理结果类型失败", None)
            }
        }
    }
}

//////// END
