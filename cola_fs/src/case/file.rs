// cola_fs/src/case/file.rs -- 可乐FS - case - 文件 - mod
// 2026/8/11 04:41 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::command::bucket::CreateBucketCmd;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 文件对象 用例
/// * `desc`: `FS - 文件对象用例`
pub struct FileCase;

impl FileCase {
    //

    ////////

    /// # 1. [CASE] - 创建存储桶
    /// * `desc`: `业务编排 - 调用 ctx 的 trait 实现`
    pub async fn case_add_bucket(
        _uid: i64,
        cmd: CreateBucketCmd,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        // 1. 调用 adapter 创建存储桶（通过 ctx 的 trait）
        let bucket_entity = ctx.fs.bucket.add.create_bucket(cmd).await?;

        info!(
            "[🗣️ CASE] - ✅️ 存储桶创建成功: bucket_id={}",
            bucket_entity.id
        );

        Ok(serde_json::to_value(&bucket_entity)?)
    }

    ////////

    /// # 2. [CASE] - 查询存储桶
    /// * `desc`: `业务编排 - 按 app_id 查询`
    pub async fn case_get_bucket(app_id: String, ctx: &AppContext) -> Result<serde_json::Value> {
        // 1. 调用 adapter 查询存储桶
        let bucket_entity = ctx
            .fs
            .bucket
            .get
            .get_bucket_by_app_id(&app_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("存储桶不存在: {}", app_id))?;

        info!("[🗣️ CASE] - ✅️ 存储桶查询成功: app_id={}", app_id);

        Ok(serde_json::to_value(&bucket_entity)?)
    }
}

//////// END
