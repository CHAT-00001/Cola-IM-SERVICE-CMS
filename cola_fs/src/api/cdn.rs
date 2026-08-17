// cola_fs/src/api/cdn.rs
// 🌐 网关 - FS - CDN域名
// 2026/8/11 04:40 Created.

////////

use crate::case::cdn::CdnCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_fs::command::cdn::{CreateCdnDomainCmd, UpdateCdnDomainCmd};
use port::app::ctx::AppContext;

////////

/// # [API HANDLER]
/// * `desc`: `CDN域名 API`
pub struct CdnApi;

impl CdnApi {
    //

    ////////

    /// # 1. [API] - 创建CDN域名
    /// * `desc`: `权限检查 → 调用 CASE 层做业务编排`
    pub async fn api_add_cdn(
        uid: i64,             // 操作者 ID
        cmd: CreateCdnDomainCmd, // 创建命令
        ctx: &AppContext,     // 全局上下文
    ) -> AppData<serde_json::Value> {
        // 权限检查：TODO - 待权限系统完成
        // TODO: verify_admin_permission(uid)?

        // 调用 CASE 层做业务编排
        match CdnCase::case_add_cdn(uid, cmd, ctx).await {
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
    pub async fn api_update_cdn(
        uid: i64, // 操作者 ID
        cdn_id: i64, // CDN 域名 ID
        cmd: UpdateCdnDomainCmd, // 更新命令
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        match CdnCase::case_update_cdn(uid, cdn_id, cmd, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 更新CDN域名失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 3. [API] - 更新 CDN 状态
    pub async fn api_change_cdn_status(
        uid: i64, // 操作者 ID
        cdn_id: i64, // CDN 域名 ID
        status: i16, // 状态码
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        match CdnCase::case_change_cdn_status(uid, cdn_id, status, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 更新CDN状态失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 4. [API] - 删除 CDN 域名
    pub async fn api_delete_cdn(
        uid: i64, // 操作者 ID
        cdn_id: i64, // CDN 域名 ID
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        match CdnCase::case_delete_cdn(cdn_id, ctx).await {
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
    pub async fn api_get_cdn(
        app_id: String,   // 应用 ID
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {

        // Call CASE ..
        match CdnCase::case_get_cdn(app_id, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 查询CDN域名成功");
                AppData::ok(data)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 查询CDN域名失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 6. [API] - 分页查询 CDN 域名列表
    pub async fn api_get_cdn_list(
        app_id: Option<String>, // 可选应用 ID
        limit: i64, // 分页数量
        offset: i64, // 分页偏移
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        match CdnCase::case_get_cdn_list(app_id, limit, offset, ctx).await {
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

    /// # 6. [API] - 按存储桶查询 CDN 域名
    pub async fn api_get_cdn_by_bucket_id(
        bucket_id: i64, // 存储桶 ID
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        match CdnCase::case_get_cdn_by_bucket_id(bucket_id, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 按存储桶查询CDN失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 7. [API] - 按 CDN ID 查询域名
    pub async fn api_get_cdn_by_id(
        cdn_id: i64, // CDN 域名 ID
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        match CdnCase::case_get_cdn_by_id(cdn_id, ctx).await {
            Ok(data) => AppData::ok(data),
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 按ID查询CDN失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }
}

//////// END
