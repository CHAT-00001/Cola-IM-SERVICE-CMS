// cola_fs/src/case/media.rs -- 可乐FS - 用例层 - 媒体对象
// 2026/8/11 04:41 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::command::bucket::CreateBucketCmd;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 媒体对象
/// * `desc`: `FS - 媒体对象 用例`
pub struct MediaCase;

impl MediaCase {
    //

    ////////

    /// # 1. [CASE] - 创建媒体对象
    /// * `desc`: `业务编排 - 调用 ctx 的 trait 实现`
    pub async fn case_add_bucket(
        _uid: i64,
        cmd: CreateBucketCmd,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        // 1. 调用 adapter 创建媒体对象（通过 ctx 的 trait）
        let bucket_entity = ctx.fs.bucket.add.create_bucket(cmd).await?;

        info!(
            "[🗣️ CASE] - ✅️ 媒体对象创建成功: bucket_id={}",
            bucket_entity.id
        );

        Ok(serde_json::to_value(&bucket_entity)?)
    }

    ////////

    /// # 2. [CASE] - 查询媒体对象
    /// * `desc`: `业务编排 - 按 app_id 查询`
    pub async fn case_get_bucket(app_id: String, ctx: &AppContext) -> Result<serde_json::Value> {
        // 1. 调用 adapter 查询媒体对象
        let bucket_entity = ctx
            .fs
            .bucket
            .get
            .get_bucket_by_app_id(&app_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("媒体对象不存在: {}", app_id))?;

        info!("[🗣️ CASE] - ✅️ 媒体对象查询成功: app_id={}", app_id);

        Ok(serde_json::to_value(&bucket_entity)?)
    }
}

//////// END
