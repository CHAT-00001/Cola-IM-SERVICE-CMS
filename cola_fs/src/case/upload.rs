// cola_fs/src/case/upload.rs -- 可乐FS - 用例层 - 文件上传
// 2026/8/14 13:00 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_fs::command::file::CreateFileCmd;
use cola_data::cola_fs::command::media::{BatchCreateMediaCmd, CreateMediaCmd};
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 文件上传
/// * `desc`: `FS - 文件对象上传`
pub struct FsUploadCase;

impl FsUploadCase {
    //

    ////////

    /// # 1. [CASE] - 获取上传密钥
    /// * `desc`: `根据 app_id 查询桶配置，生成预签名 URL`
    pub async fn case_get_upload_key(
        uid: i64,
        app_id: String,
        file_name: String,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        // 1. 查询桶配置
        let bucket = ctx
            .fs
            .bucket
            .get
            .get_bucket_by_app_id(&app_id)
            .await?
            .ok_or_else(|| anyhow!("Bucket not found for app_id: {}", app_id))?;

        // 2. 生成 object key（例如：2026/8/14/user_123/file_name.ext）
        let timestamp = chrono::Utc::now().format("%Y/%m/%d").to_string();
        let object_key = format!("{}/user_{}/{}", timestamp, uid, file_name);

        // 3. 生成预签名 URL
        let presigned_url = ctx
            .fs
            .bucket
            .add
            .get_presigned_url(uid, bucket.id, &object_key, 600)
            .await?;

        info!(
            "[🗣️ CASE] - ✅️ 生成上传密钥成功: uid={}, app_id={}",
            uid, app_id
        );

        Ok(serde_json::json!({
            "presigned_url": presigned_url,
            "object_key": object_key,
            "bucket": bucket.bucket,
            "expired_at": chrono::Utc::now().checked_add_signed(chrono::Duration::hours(1)),
            "expires_in": 600,
        }))
    }

    ////////

    /// # 2. [CASE] - 创建临时文件记录
    pub async fn case_create_temp_file(
        uid: i64,
        cmd: CreateFileCmd,
        ctx: &AppContext,
    ) -> Result<cola_data::cola_fs::entity::file::FsFileEntity> {
        let file = ctx
            .fs
            .file
            .add
            .create_temp_file(
                uid,
                cmd.app_id.unwrap_or_default(),
                cmd.bucket_key,
                cmd.object_key,
                cmd.original_name.unwrap_or_default(),
                cmd.file_size,
                cmd.mime_type,
                7, // 默认 7 天过期
            )
            .await?;

        info!(
            "[🗣️ CASE] - ✅️ 创建临时文件成功: uid={}, file_id={}",
            uid, file.id
        );

        Ok(file)
    }

    ////////

    /// # 3. [CASE] - 创建媒体资源
    pub async fn case_create_media(
        uid: i64,
        cmd: CreateMediaCmd,
        ctx: &AppContext,
    ) -> Result<cola_data::cola_fs::entity::media::MediaEntity> {
        // 1. 参数校验
        if cmd.media_type < 1 || cmd.media_type > 4 {
            return Err(anyhow!("Invalid media_type: {}", cmd.media_type));
        }

        // 2. 创建媒体记录
        let media = ctx.fs.media.add.create_media(uid, cmd).await?;

        info!(
            "[🗣️ CASE] - ✅️ 创建媒体资源成功: uid={}, media_id={}",
            uid, media.id
        );

        Ok(media)
    }

    ////////

    /// # 4. [CASE] - 批量创建媒体资源
    /// * `desc`: `校验通用 UGC 媒体结构后逐项创建，失败时不返回部分结果`
    pub async fn case_batch_create_media(
        uid: i64,
        cmd: BatchCreateMediaCmd,
        ctx: &AppContext,
    ) -> Result<Vec<cola_data::cola_fs::entity::media::MediaEntity>> {
        if cmd.medias.is_empty() || cmd.medias.len() > 20 {
            return Err(anyhow!("媒体数量必须在 1 到 20 之间"));
        }

        let mut result = Vec::with_capacity(cmd.medias.len());
        for media_cmd in cmd.medias {
            Self::validate_media_files(&media_cmd)?;
            let media = ctx.fs.media.add.create_media(uid, media_cmd).await?;
            result.push(media);
        }
        info!(
            "[🗣️ CASE] - ✅️ 批量媒体创建成功: uid={}, count={}",
            uid,
            result.len()
        );
        Ok(result)
    }

    ////////

    fn validate_media_files(cmd: &CreateMediaCmd) -> Result<()> {
        if !(1..=6).contains(&cmd.media_type) {
            return Err(anyhow!("不支持的媒体类型: {}", cmd.media_type));
        }
        match cmd.media_type {
            3 if cmd.main_file_id.is_none() || cmd.aux_file_id.is_none() => {
                Err(anyhow!("LivePhoto 必须同时提供静态图片和动态视频文件"))
            }
            1 | 2 | 4 | 5 | 6 if cmd.main_file_id.is_none() => {
                Err(anyhow!("媒体类型 {} 必须提供主文件", cmd.media_type))
            }
            _ => Ok(()),
        }
    }

    ////////

    /// # 4. [CASE] - 标记文件为正式
    /// * `desc`: `UGC 发布后调用，将临时文件转为正式文件`
    pub async fn case_mark_files_official(
        uid: i64,
        file_ids: Vec<i64>,
        ref_table: String,
        ref_id: i64,
        ctx: &AppContext,
    ) -> Result<u64> {
        if file_ids.is_empty() {
            return Ok(0);
        }

        let count = ctx
            .fs
            .file
            .manage
            .mark_files_as_official(uid, file_ids, ref_table, ref_id)
            .await?;

        info!("[🗣️ CASE] - ✅️ 标记文件为正式成功: count={}", count);

        Ok(count)
    }
}

//////// END
