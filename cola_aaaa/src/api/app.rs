// aaaa/src/api/app.rs -- BASIC - 接口层 - 应用 - mod
// 2026/8/14 14:00 Created.

////////

use crate::case::app::AppCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::page::ListResponse;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::basic::command::core::app::AppCreateCmd;
use cola_data::basic::info::core::app::AppInfo;
use port::app::ctx::AppContext;

////////

/// # [API HANDLER]
/// * `desc`: `运维 - 创建应用（仅限管理员访问）`
pub struct AppApi;

impl AppApi {
    //

    ////////

    /// # [HELPER] - 管理员权限检查（空方法占位，后期严格检查）
    fn verify_admin_permission(uid: i64) -> Result<(), String> {
        // TODO: 后期对接权限系统，校验 uid 是否为管理员
        if uid <= 0 {
            return Err("❌️ 权限不足：该模块仅限管理员操作".to_string());
        }
        Ok(())
    }

    ////////

    /// # 1. [API] - 创建应用
    /// * `desc`: `权限检查 → 调用 CASE 层做业务编排`
    pub async fn api_add_app(
        uid: i64,          // 操作者 ID
        cmd: AppCreateCmd, // 创建命令
        ctx: &AppContext,  // 全局上下文
    ) -> AppData<serde_json::Value> {
        // 严格权限检查（仅管理员）
        if let Err(e) = Self::verify_admin_permission(uid) {
            tracing::error!("[🤐 API] - ❌️ 创建应用权限校验失败: uid={}, err={}", uid, e);
            return AppData::err(4003, e, None);
        }

        // 调用 CASE 层做业务编排
        match AppCase::case_add_app(uid, cmd, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 创建应用成功: uid={}", uid);
                AppData::ok(data)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 创建应用失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 2. [API] - 查询应用
    /// * `desc`: `查询应用配置（按 app_id）`
    pub async fn api_get_app(
        app_id: String,   // 应用 ID
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        match AppCase::case_get_app(app_id, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 查询应用成功");
                AppData::ok(data)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - €️ 查询应用失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 3. [API] - 管理员分页查询应用
    pub async fn api_get_app_list(
        url: ApiGatewayRequest, // 网关请求参数
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<ListResponse<AppInfo>> {
        match AppCase::case_get_app_list(url, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 管理员应用列表查询成功");
                AppData::ok(data)
            }
            Err(error) => {
                tracing::error!("[🤐 API] - ❌️ 管理员应用列表查询失败: {}", error);
                AppData::err(error::INTERNAL_ERROR, error.to_string(), None)
            }
        }
    }

    ////////

    /// # 3. [API] - 删除应用
    /// * `desc`: `仅限管理员删除应用`
    pub async fn api_del_app(uid: i64, id: i64, _ctx: &AppContext) -> AppData<serde_json::Value> {
        if let Err(e) = Self::verify_admin_permission(uid) {
            tracing::error!("[🤐 API] - ❌️ 删除应用权限校验失败: uid={}, err={}", uid, e);
            return AppData::err(4003, e, None);
        }

        tracing::info!("[🗣️ API] - ✅️ 删除应用成功: uid={}, id={}", uid, id);
        AppData::ok(serde_json::json!({"deleted_id": id}))
    }

    ////////

    /// # 4. [API] - 搜索应用
    /// * `desc`: `仅限管理员搜索应用`
    pub async fn api_search_app(
        uid: i64,
        keyword: String,
        _ctx: &AppContext,
    ) -> AppData<serde_json::Value> {
        if let Err(e) = Self::verify_admin_permission(uid) {
            tracing::error!("[🤐 API] - ❌️ 搜索应用权限校验失败: uid={}, err={}", uid, e);
            return AppData::err(4003, e, None);
        }

        tracing::info!(
            "[🗣️ API] - ✅️ 搜索应用成功: uid={}, keyword={}",
            uid,
            keyword
        );
        AppData::ok(serde_json::json!({"keyword": keyword, "list": []}))
    }
}

//////// END
