// cola_basic/src/api/node.rs -- BASIC - 接口层 - 节点接口
// 2026/5/11 01:40 Created.

////////

use crate::case::node::NodeCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::basic::command::core::node::{NodeCreateCmd, NodeUpdateCmd};
use port::app::ctx::AppContext;

////////

/// # [API HANDLER]
/// * `desc`: `节点接口`
pub struct NodeApi;

impl NodeApi {
    //

    ////////

    /// # 1. [API] - 创建CDN域名
    /// * `desc`: `权限检查 → 调用 CASE 层做业务编排`
    pub async fn api_add_node(
        uid: i64,           // 操作者 ID
        cmd: NodeCreateCmd, // 创建命令
        ctx: &AppContext,   // 全局上下文
    ) -> AppData<serde_json::Value> {
        // 权限检查：TODO - 待权限系统完成
        // TODO: verify_admin_permission(uid)?

        // 💡 - CASE - 调用 CASE 层做业务编排
        match NodeCase::case_add_node(uid, cmd, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 创建CDN域名成功: uid={}", uid);
                AppData::ok(data)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 创建CDN域名失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 2. [API] - 更新 CDN 域名
    pub async fn api_update_node(
        uid: i64,           // 操作者 ID
        node_id: i64,       // CDN 域名 ID
        cmd: NodeUpdateCmd, // 更新命令
        ctx: &AppContext,   // 全局上下文
    ) -> AppData<serde_json::Value> {
        ////////

        // 💡 - CASE - 用例层
        match NodeCase::case_update_node(uid, node_id, cmd, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 更新CDN域名失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 3. [API] - 更新 CDN 状态
    pub async fn api_change_node_status(
        uid: i64,         // 操作者 ID
        node_id: i64,     // CDN 域名 ID
        status: i16,      // 状态码
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        ////////

        // 💡 - CASE - 用例层
        match NodeCase::case_change_node_status(uid, node_id, status, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 更新CDN状态失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 4. [API] - 删除 CDN 域名
    pub async fn api_delete_node(
        uid: i64,         // 操作者 ID
        node_id: i64,     // CDN 域名 ID
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        ////////

        // 💡 - CASE - 用例层
        match NodeCase::case_delete_node(node_id, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 删除CDN域名失败: uid={}, {}", uid, e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 2. [API] - 查询CDN域名
    /// * `desc`: `查询CDN域名配置（按 app_id）`
    pub async fn api_get_node(
        node_id: i64,     // 节点 ID
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        ////////

        // 💡 - CASE - 用例层
        match NodeCase::case_get_node_by_id(node_id, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 查询NODE节点成功");
                AppData::ok(data)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 查询NODE节点失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 6. [API] - 分页查询 CDN 域名列表
    pub async fn api_get_node_list(
        app_id: Option<String>, // 可选应用 ID
        limit: i64,             // 分页数量
        offset: i64,            // 分页偏移
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<serde_json::Value> {
        ////////

        // 💡 - CASE - 用例层
        match NodeCase::case_get_node_list(app_id, limit, offset, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ CDN列表查询成功");
                AppData::ok(data)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ CDN列表查询失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 6. [API] - 按区域查询NODE域名
    pub async fn api_get_node_by_region_id(
        region_id: i64,         // 存储桶 ID
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<serde_json::Value> {
        ////////

        // 💡 - CASE - 用例层
        match NodeCase::case_get_node_by_region_id(region_id, url.limit, url.offset, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 按存储桶查询CDN失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 7. [API] - 按 NODE ID 查询域名
    pub async fn api_get_node_by_id(
        node_id: i64,     // CDN 域名 ID
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        ////////

        // 💡 - CASE - 用例层
        match NodeCase::case_get_node_by_id(node_id, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 按节点ID查询NODE失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }
}

//////// END
