// cola_fs/src/api/upload.rs
// 🌐 网关 - FS - 上传
// 2026/8/14 13:00 Created.

////////

use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_fs::command::file::CreateFileCmd;
use cola_data::cola_fs::command::media::CreateMediaCmd;
use port::app::ctx::AppContext;
use crate::case::upload::FsUploadCase;

////////

pub struct FsUploadApi;

impl FsUploadApi {
    ////////

    /// # [API] - 获取 S3 上传密钥
    /// * `desc`: `根据 app_id 生成预签名 URL`
    pub async fn api_get_upload_key(
        uid: i64,                   // 操作者 ID
        app_id: String,             // 应用 ID
        file_name: String,          // 文件名
        ctx: &AppContext,           // 全局上下文
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
                tracing::info!("[🗣️ API] - ✅️ 创建临时文件成功: uid={}, file_id={}", uid, file.id);
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
                tracing::info!("[🗣️ API] - ✅️ 创建媒体资源成功: uid={}, media_id={}", uid, media.id);
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
}

//////// END

