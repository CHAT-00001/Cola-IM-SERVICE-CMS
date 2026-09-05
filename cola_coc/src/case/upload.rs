// cola_video/src/case/upload.rs
// 🗣️ 短视频 - CASE - 专属上传会话
// 2026/8/17 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_fs::command::upload::{
    CreateUploadSessionRequest, UploadFileRequest, UploadPolicy,
};
use cola_data::cola_video::command::video::upload::{VideoUploadFileCmd, VideoUploadSessionCmd};
use port::app::ctx::AppContext;
use serde_json::Value;

////////

const VIDEO_APP_ID: &str = "ugc-video";

/// # [CASE] - 短视频上传会话
/// * `desc`: `短视频业务固定使用 ugc-video，不允许客户端选择 Bucket`
pub struct VideoUploadCase;

impl VideoUploadCase {
    ////////

    /// # 1. [CASE] - 申请短视频封面和视频上传凭证
    /// * `desc`: `服务端生成会话和两个 Object Key，并为每个文件生成短期 PUT URL`
    pub async fn case_create_upload_session(
        uid: i64,
        cmd: VideoUploadSessionCmd,
        ctx: &AppContext,
    ) -> Result<Value> {
        if cmd.idempotency_key.trim().is_empty() {
            return Err(anyhow!("缺少上传会话幂等键"));
        }
        let request = CreateUploadSessionRequest {
            app_id: VIDEO_APP_ID.to_string(),
            ugc_type: "video".to_string(),
            idempotency_key: cmd.idempotency_key,
            files: vec![
                Self::to_upload_file("cover", cmd.cover),
                Self::to_upload_file("main", cmd.main),
            ],
        };
        let policy =
            UploadPolicy::by_app_id(VIDEO_APP_ID).ok_or_else(|| anyhow!("短视频上传策略不存在"))?;
        ctx.fs.upload.create_session(uid, request, policy).await
    }

    ////////

    fn to_upload_file(role: &str, file: VideoUploadFileCmd) -> UploadFileRequest {
        UploadFileRequest {
            role: role.to_string(),
            file_name: file.file_name,
            mime_type: file.mime_type,
            file_size: file.file_size,
            file_hash: file.file_hash,
        }
    }
}

//////// END
