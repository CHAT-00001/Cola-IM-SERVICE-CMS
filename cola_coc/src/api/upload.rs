// cola_video/src/api/upload.rs
// 🌐 短视频 - API - 专属上传会话
// 2026/8/17 Created.

////////

use crate::case::upload::VideoUploadCase;
use cola_data::app::{data::AppData, error};
use cola_data::cola_video::command::video::upload::VideoUploadSessionCmd;
use port::app::ctx::AppContext;

////////

/// # [API] - 短视频上传
/// * `desc`: `业务 API 固定 app_id=ugc-video，客户端不能指定存储桶`
pub struct VideoUploadApi;

impl VideoUploadApi {
    ////////

    /// # 1. [API] - 创建短视频上传会话
    /// * `desc`: `一次返回封面和主视频两个上传凭证`
    pub async fn api_create_upload_session(
        uid: i64,
        cmd: VideoUploadSessionCmd,
        ctx: &AppContext,
    ) -> AppData<serde_json::Value> {
        match VideoUploadCase::case_create_upload_session(uid, cmd, ctx).await {
            Ok(data) => {
                tracing::info!("[🗣️ API] - ✅️ 短视频上传凭证申请成功: uid={}", uid);
                AppData::ok(data)
            }
            Err(error_value) => {
                tracing::error!("[🤐 API] - ❌️ 短视频上传凭证申请失败: {}", error_value);
                AppData::err(error::PARAM_ERROR, error_value.to_string(), None)
            }
        }
    }
}

//////// END
