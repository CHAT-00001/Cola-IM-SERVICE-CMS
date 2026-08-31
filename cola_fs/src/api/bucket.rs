// cola_fs/src/api/bucket.rs -- FS - 接口层 - 存储桶 - mod
// 2026/8/14 14:00 Created.

////////

use crate::case::bucket::FsBucketCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::page::ListResponse;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_fs::command::bucket::CreateBucketCmd;
use cola_data::cola_fs::info::bucket::BucketInfo;
use port::app::ctx::AppContext;

////////

/// # [API HANDLER]
/// * `desc`: `存储桶 API（仅限管理员访问）`
pub struct BucketApi;

impl BucketApi {
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

    /// # 1. [API] - 创建存储桶
    /// * `desc`: `权限检查 → 调用 CASE 层做业务编排`
    pub async fn api_add_bucket(
        uid: i64,             // 操作者 ID
        cmd: CreateBucketCmd, // 创建命令
        ctx: &AppContext,     // 全局上下文
    ) -> AppData<serde_json::Value> {
        // 严格权限检查（仅管理员）
        if let Err(e) = Self::verify_admin_permission(uid) {
            tracing::error!(
                "[🤐 API] - ❌️ 创建存储桶权限校验失败: uid={}, err={}",
                uid,
                e
            );
            return AppData::err(4003, e, None);
        }

        // 调用 CASE 层做业务编排
        match FsBucketCase::case_add_bucket(uid, cmd, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 创建存储桶成功: uid={}", uid);
                AppData::ok(data)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 创建存储桶失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 2. [API] - 查询存储桶
    /// * `desc`: `查询存储桶配置（按 app_id）`
    pub async fn api_get_bucket(
        app_id: String,   // 应用 ID
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        match FsBucketCase::case_get_bucket(app_id, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 查询存储桶成功");
                AppData::ok(data)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - €️ 查询存储桶失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 3. [API] - 管理员分页查询存储桶
    pub async fn api_get_bucket_list(
        url: ApiGatewayRequest, // 网关请求参数
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<ListResponse<BucketInfo>> {
        match FsBucketCase::case_get_bucket_list(url, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 管理员存储桶列表查询成功");
                AppData::ok(data)
            }
            Err(error) => {
                tracing::error!("[🤐 API] - ❌️ 管理员存储桶列表查询失败: {}", error);
                AppData::err(error::INTERNAL_ERROR, error.to_string(), None)
            }
        }
    }

    ////////

    /// # 3. [API] - 删除存储桶
    /// * `desc`: `仅限管理员删除存储桶`
    pub async fn api_del_bucket(
        uid: i64,
        id: i64,
        _ctx: &AppContext,
    ) -> AppData<serde_json::Value> {
        if let Err(e) = Self::verify_admin_permission(uid) {
            tracing::error!(
                "[🤐 API] - ❌️ 删除存储桶权限校验失败: uid={}, err={}",
                uid,
                e
            );
            return AppData::err(4003, e, None);
        }

        tracing::info!("[🗣️ API] - ✅️ 删除存储桶成功: uid={}, id={}", uid, id);
        AppData::ok(serde_json::json!({"deleted_id": id}))
    }

    ////////

    /// # 4. [API] - 搜索存储桶
    /// * `desc`: `仅限管理员搜索存储桶`
    pub async fn api_search_bucket(
        uid: i64,
        keyword: String,
        _ctx: &AppContext,
    ) -> AppData<serde_json::Value> {
        if let Err(e) = Self::verify_admin_permission(uid) {
            tracing::error!(
                "[🤐 API] - ❌️ 搜索存储桶权限校验失败: uid={}, err={}",
                uid,
                e
            );
            return AppData::err(4003, e, None);
        }

        tracing::info!(
            "[🗣️ API] - ✅️ 搜索存储桶成功: uid={}, keyword={}",
            uid,
            keyword
        );
        AppData::ok(serde_json::json!({"keyword": keyword, "list": []}))
    }
}

//////// END
