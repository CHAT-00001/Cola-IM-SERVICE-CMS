// cola_fs/src/api/file.rs -- FS - 接口层 - 文件
// 2026/8/16 01:18 Created.

////////

// //cdn.rs
//
// 2026/8/11 04:40 Created.

////////

use crate::case::bucket::FsBucketCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_fs::command::bucket::CreateBucketCmd;
use port::app::ctx::AppContext;

////////

/// # [API HANDLER]
/// * `desc`: `文件对象 API`
pub struct FileApi;

impl FileApi {
    //

    ////////

    /// # 1. [API] - 创建文件对象
    /// * `desc`: `权限检查 → 调用 CASE 层做业务编排`
    pub async fn api_add_file(
        uid: i64,             // 操作者 ID
        cmd: CreateBucketCmd, // 创建命令
        ctx: &AppContext,     // 全局上下文
    ) -> AppData<serde_json::Value> {
        // 权限检查：TODO - 待权限系统完成
        // TODO: verify_admin_permission(uid)?

        // 调用 CASE 层做业务编排
        match FsBucketCase::case_add_bucket(uid, cmd, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 创建文件对象成功: uid={}", uid);
                AppData::ok(data)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 创建文件对象失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }

    ////////

    /// # 2. [API] - 查询文件对象
    /// * `desc`: `查询文件对象配置（按 app_id）`
    pub async fn api_get_file(
        app_id: String,   // 应用 ID
        ctx: &AppContext, // 全局上下文
    ) -> AppData<serde_json::Value> {
        match FsBucketCase::case_get_bucket(app_id, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 查询文件对象成功");
                AppData::ok(data)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 查询文件对象失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, &e.to_string(), None)
            }
        }
    }
}

//////// END
