// aaaa/src/api/binding.rs -- BASIC - 接口层 - 应用绑定接口
// 2026/09/30 11:30 Created.

////////

use crate::case::binding::BindingCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::page::ListResponse;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::basic::command::core::app::AppCreateCmd;
use cola_data::basic::info::core::app::AppInfo;
use port::app::ctx::AppContext;

////////

/// # [API HANDLER] - 应用绑定
/// * `desc`: `当前阶段使用 aaaa.app 表承载 app_id 与存储配置绑定`
pub struct BindingApi;

impl BindingApi {
    /// # 1. [API] - 创建应用绑定
    /// * `desc`: `创建 app_id 与当前应用存储配置的绑定记录`
    pub async fn api_add_binding(
        uid: i64,
        cmd: AppCreateCmd,
        ctx: &AppContext,
    ) -> AppData<serde_json::Value> {
        match BindingCase::case_add_binding(uid, cmd, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(error_value) => AppData::err(error::INTERNAL_ERROR, error_value.to_string(), None),
        }
    }

    /// # 2. [API] - 查询应用绑定
    /// * `desc`: `按 app_id 查询应用及其当前存储配置绑定`
    pub async fn api_get_binding(app_id: String, ctx: &AppContext) -> AppData<serde_json::Value> {
        match BindingCase::case_get_binding(app_id, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(error_value) => AppData::err(error::INTERNAL_ERROR, error_value.to_string(), None),
        }
    }

    /// # 3. [API] - 分页查询应用绑定
    /// * `desc`: `应用绑定列表复用当前 App 分页查询能力`
    pub async fn api_get_binding_list(
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<ListResponse<AppInfo>> {
        match BindingCase::case_get_binding_list(url, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(error_value) => AppData::err(error::INTERNAL_ERROR, error_value.to_string(), None),
        }
    }

    /// # 4. [API] - 删除应用绑定
    /// * `desc`: `当前阶段对 App 记录执行逻辑删除`
    pub async fn api_delete_binding(
        uid: i64,
        id: i64,
        ctx: &AppContext,
    ) -> AppData<serde_json::Value> {
        match BindingCase::case_delete_binding(uid, id, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(error_value) => AppData::err(error::INTERNAL_ERROR, error_value.to_string(), None),
        }
    }
}

//////// END
