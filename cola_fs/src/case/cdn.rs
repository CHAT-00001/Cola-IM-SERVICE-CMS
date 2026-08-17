// cola_fs/src/case/cdn.rs
// 可乐FS - 用例层 - CDN域名
// 2026/8/11 04:41 Created.

////////

use anyhow::Result;
use tracing::info;
use cola_data::cola_fs::command::cdn::{CreateCdnDomainCmd, UpdateCdnDomainCmd};
use port::app::ctx::AppContext;

////////

/// # [CASE] - CDN域名 用例
/// * `desc`: `FS CDN域名`
pub struct CdnCase;

impl CdnCase {
    //

    ////////

    /// # 1. [CASE] - 创建CDN域名
    /// * `desc`: `业务编排 - 调用 ctx 的 trait 实现`
    pub async fn case_add_cdn(
        _uid: i64,
        cmd: CreateCdnDomainCmd,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        let info = ctx.fs.cdn.config.create(cmd).await?;

        info!("[🗣️ CASE] - ✅️ CDN域名创建成功: cdn_id={}", info.id);

        Ok(serde_json::to_value(info)?)
    }

    ////////

    /// # 2. [CASE] - 更新CDN域名
    /// * `desc`: `业务编排 - 调用 ctx 的 trait 实现`
    pub async fn case_update_cdn(
        _uid: i64,
        cdn_id: i64, // CDN域名 ID
        cmd: UpdateCdnDomainCmd,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        let info = ctx.fs.cdn.config.update(cdn_id, cmd).await?;

        info!("[🗣️ CASE] - ✅️ CDN域名更新成功: cdn_id={}", info.id);

        Ok(serde_json::to_value(info)?)
    }

    ////////

    /// # 3. [CASE] - 更新CDN域名状态
    /// * `desc`: `业务编排 - 调用 ctx 的 trait 实现`
    pub async fn case_change_cdn_status(
        _uid: i64,
        cdn_id: i64,
        status: i16,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        let info = ctx.fs.cdn.config.update_status(cdn_id, status).await?;

        info!("[🗣️ CASE] - ✅️ CDN状态更新成功: cdn_id={}, status={}", cdn_id, status);

        Ok(serde_json::to_value(info)?)
    }

    ////////

    /// # 4. [CASE] - 逻辑删除CDN域名
    /// * `desc`: `业务编排 - 按 cdn_id 查询`
    pub async fn case_delete_cdn(
        cdn_id: i64, // CDN域名 ID
        ctx: &AppContext,  // 全局上下文
    ) -> Result<serde_json::Value> {
        let affected = ctx.fs.cdn.config.delete(cdn_id).await?;
        if affected == 0 {
            return Err(anyhow::anyhow!("CDN域名不存在: {}", cdn_id));
        }

        info!("[🗣️ CASE] - ✅️ CDN域名删除成功: cdn_id={}", cdn_id);

        Ok(serde_json::json!({"cdn_id": cdn_id, "deleted": true}))
    }

    ////////

    /// # 5. [CASE] - 查询CDN域名列表
    /// * `desc`: `业务编排 - 按 app_id 查询`
    pub async fn case_get_cdn(
        app_id: String,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        let bucket = ctx.fs.bucket.get.get_bucket_by_app_id(&app_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("存储桶不存在: {}", app_id))?;

        info!("[🗣️ CASE] - ✅️ Bucket CDN查询成功: app_id={}", app_id);

        Ok(serde_json::json!({
            "app_id": bucket.app_id,
            "bucket": bucket.bucket,
            "cdn_domain": bucket.cdn_domain
        }))
    }

    ////////

    /// # 6. [CASE] - 分页查询 CDN 域名列表
    /// * `desc`: `无条件查询全部 CDN，支持 app_id 可选筛选`
    pub async fn case_get_cdn_list(
        app_id: Option<String>, // 可选应用 ID
        limit: i64, // 分页数量
        offset: i64, // 分页偏移
        ctx: &AppContext, // 全局上下文
    ) -> Result<serde_json::Value> {
        let (list, total) = ctx
            .fs
            .bucket
            .list
            .admin_find_page(app_id.as_deref(), None, limit, offset)
            .await?;

        info!(
            "[🗣️ CASE] - ✅️ CDN列表查询成功: app_id={:?}, count={}, total={}",
            app_id,
            list.len(),
            total
        );

        Ok(serde_json::json!({
            "list": list,
            "total": total,
            "limit": limit,
            "offset": offset
        }))
    }

    ////////

    /// # 7. [CASE] - 根据桶ID 查询CDN域名
    /// * `desc`: `业务编排 - 按 bucket_id 查询`
    pub async fn case_get_cdn_by_bucket_id(
        bucket_id: i64,  // 存储桶 ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<serde_json::Value> {
        let bucket = ctx.fs.bucket.get.get_bucket_by_id(bucket_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("存储桶不存在: {}", bucket_id))?;

        info!("[🗣️ CASE] - ✅️ Bucket CDN查询成功: bucket_id={}", bucket_id);

        Ok(serde_json::json!({
            "bucket_id": bucket.id,
            "app_id": bucket.app_id,
            "bucket": bucket.bucket,
            "cdn_domain": bucket.cdn_domain
        }))
    }

    ////////

    /// # 8. [CASE] - 根据CDN域名 ID 查询CDN域名
    /// * `desc`: `业务编排 - 按 cdn_id 单个查询`
    pub async fn case_get_cdn_by_id(
        cdn_id: i64,  // 存储桶 ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<serde_json::Value> {
        // 1. 调用 adapter 查询CDN域名
        let info = ctx.fs.cdn.config.find_by_id(cdn_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("CDN域名不存在: {}", cdn_id))?;

        info!("[🗣️ CASE] - ✅️ CDN域名查询成功: app_id={}", cdn_id);

        Ok(serde_json::to_value(info)?)
    }
}

//////// END
