// cola_fs/src/api/upload.rs -- FS - 接口层 - 上传 - mod
// 2026/8/14 13:00 Created.

////////

use crate::case::upload::FsUploadCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_fs::command::file::CreateFileCmd;
use cola_data::cola_fs::command::media::{BatchCreateMediaCmd, CreateMediaCmd};
use cola_data::cola_fs::command::upload::CreateUploadSessionRequest;
use port::app::ctx::AppContext;

////////

/// # [API] - 上传接口
pub struct FsUploadApi;

impl FsUploadApi {
    //

    ////////

    /// # [API] - 创建通用上传会话
    /// * `desc`: `所有 UGC 复用，根据 app_id 策略选择 Bucket 和允许的 MIME 类型`
    pub async fn api_create_upload_session(
        uid: i64,
        request: CreateUploadSessionRequest,
        ctx: &AppContext,
    ) -> AppData<serde_json::Value> {
        let policy =
            match cola_data::cola_fs::command::upload::UploadPolicy::by_app_id(&request.app_id) {
                Some(policy) => policy,
                None => return AppData::err(error::PARAM_ERROR, "不支持的上传业务 app_id", None),
            };
        match ctx.fs.upload.create_session(uid, request, policy).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 通用上传会话创建成功: uid={}", uid);
                AppData::ok(data)
            }
            Err(error_value) => {
                tracing::error!("[🤐 API] - ❌️ 通用上传会话创建失败: {}", error_value);
                AppData::err(error::PARAM_ERROR, error_value.to_string(), None)
            }
        }
    }

    ////////

    /// # [API] - 获取 S3 上传密钥
    /// * `desc`: `根据 app_id 生成预签名 URL`
    pub async fn api_get_upload_key(
        uid: i64,          // 操作者 ID
        app_id: String,    // 应用 ID
        file_name: String, // 文件名
        ctx: &AppContext,  // 全局上下文
    ) -> AppData<serde_json::Value> {
        // 1. 参数验证
        if app_id.is_empty() {
            return AppData::err(error::PARAM_ERROR, "missing app_id", None);
        }
        if file_name.is_empty() {
            return AppData::err(error::PARAM_ERROR, "missing file_name", None);
        }

        // 2. 执行核心逻辑
        match FsUploadCase::case_get_upload_key(uid, app_id, file_name, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 获取上传密钥成功: uid={}", uid);
                AppData::ok(data)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 获取上传密钥失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, "获取上传密钥失败", None)
            }
        }
    }

    ////////

    /// # [API] - 创建临时文件记录
    pub async fn api_create_temp_file(
        uid: i64,
        cmd: CreateFileCmd,
        ctx: &AppContext,
    ) -> AppData<serde_json::Value> {
        match FsUploadCase::case_create_temp_file(uid, cmd, ctx).await {
            Ok(file) => {
                tracing::info!(
                    "[🗣️ API] - ✅️ 创建临时文件成功: uid={}, file_id={}",
                    uid,
                    file.id
                );
                AppData::ok(serde_json::to_value(&file).unwrap_or_default())
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 创建临时文件失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, "创建临时文件失败", None)
            }
        }
    }

    ////////

    /// # [API] - 创建媒体资源
    pub async fn api_create_media(
        uid: i64,
        cmd: CreateMediaCmd,
        ctx: &AppContext,
    ) -> AppData<serde_json::Value> {
        match FsUploadCase::case_create_media(uid, cmd, ctx).await {
            Ok(media) => {
                tracing::info!(
                    "[🗣️ API] - ✅️ 创建媒体资源成功: uid={}, media_id={}",
                    uid,
                    media.id
                );
                AppData::ok(serde_json::to_value(&media).unwrap_or_default())
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 创建媒体资源失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, "创建媒体资源失败", None)
            }
        }
    }

    ////////

    /// # [API] - 标记文件为正式
    /// * `desc`: `UGC 发布后调用，将临时文件转为正式文件`
    pub async fn api_mark_files_official(
        uid: i64,
        file_ids: Vec<i64>,
        ref_table: String,
        ref_id: i64,
        ctx: &AppContext,
    ) -> AppData<serde_json::Value> {
        match FsUploadCase::case_mark_files_official(uid, file_ids, ref_table, ref_id, ctx).await {
            Ok(count) => {
                tracing::info!("[🗣️ API] - ✅️ 标记文件为正式成功: count={}", count);
                AppData::ok(serde_json::json!({"updated": count}))
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 标记文件为正式失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, "标记文件为正式失败", None)
            }
        }
    }

    ////////

    /// # [API] - 批量创建媒体资源
    /// * `desc`: `所有 UGC 业务共用，校验后返回 Media 列表`
    pub async fn api_batch_create_media(
        uid: i64,
        cmd: BatchCreateMediaCmd,
        ctx: &AppContext,
    ) -> AppData<serde_json::Value> {
        match FsUploadCase::case_batch_create_media(uid, cmd, ctx).await {
            Ok(media) => {
                tracing::info!(
                    "[🗣️ API] - ✅️ 批量创建媒体成功: uid={}, count={}",
                    uid,
                    media.len()
                );
                AppData::ok(serde_json::json!({"list": media, "count": media.len()}))
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ 批量创建媒体失败: {}", e);
                AppData::err(error::PARAM_ERROR, &e.to_string(), None)
            }
        }
    }
}

//////// END
