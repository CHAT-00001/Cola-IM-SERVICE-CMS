// D:\rust\short-video\repo_adapter\src\fs\cdn\config.rs
// 🔌 适配器 - FS - CDN - 配置管理
// 2026/8/16 Created.

////////

use async_trait::async_trait;
use cola_data::cola_fs::command::cdn::{CreateCdnDomainCmd, UpdateCdnDomainCmd};
use cola_data::cola_fs::info::cdn::CdnDomainInfo;
use port::fs::cdn::config::CdnConfigPort;
use repository::cola_fs::pg::cnd::CdnDomainRepo;
use repository::pg_pool;

////////

/// # 1. [ADAPTER] - CDN 配置适配器
#[derive(Debug, Default, Clone)]
pub struct CdnConfigAdapter;

////////

#[async_trait]
impl CdnConfigPort for CdnConfigAdapter {
    async fn create(&self, cmd: CreateCdnDomainCmd) -> anyhow::Result<CdnDomainInfo> {
        let entity = CdnDomainRepo::create(&pg_pool(), cmd).await?;
        tracing::info!("[🔌 ADAPTER] - ✅️ CDN配置创建成功: id={}", entity.id);
        Ok(entity.into())
    }

    async fn update(&self, id: i64, cmd: UpdateCdnDomainCmd) -> anyhow::Result<CdnDomainInfo> {
        let entity = CdnDomainRepo::update(&pg_pool(), id, cmd)
            .await?
            .ok_or_else(|| anyhow::anyhow!("CDN域名不存在: {}", id))?;
        tracing::info!("[🔌 ADAPTER] - ✅️ CDN配置更新成功: id={}", id);
        Ok(entity.into())
    }

    async fn update_status(&self, id: i64, status: i16) -> anyhow::Result<CdnDomainInfo> {
        let entity = CdnDomainRepo::update_status(&pg_pool(), id, status)
            .await?
            .ok_or_else(|| anyhow::anyhow!("CDN域名不存在: {}", id))?;
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ CDN状态更新成功: id={}, status={}",
            id,
            status
        );
        Ok(entity.into())
    }

    async fn find_by_app_id(&self, app_id: &str) -> anyhow::Result<Option<CdnDomainInfo>> {
        Ok(CdnDomainRepo::find_by_app_id(&pg_pool(), app_id)
            .await?
            .map(Into::into))
    }

    async fn find_by_bucket_key(
        &self,
        app_id: Option<&str>,
        bucket_key: &str,
    ) -> anyhow::Result<Option<CdnDomainInfo>> {
        Ok(
            CdnDomainRepo::find_by_bucket_key(&pg_pool(), app_id, bucket_key)
                .await?
                .map(Into::into),
        )
    }

    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<CdnDomainInfo>> {
        Ok(CdnDomainRepo::find_by_id(&pg_pool(), id)
            .await?
            .map(Into::into))
    }

    async fn list(
        &self,
        app_id: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<CdnDomainInfo>, i64)> {
        let (entities, total) =
            CdnDomainRepo::admin_find_page(&pg_pool(), app_id, limit, offset).await?;
        let list: Vec<CdnDomainInfo> = entities.into_iter().map(Into::into).collect();
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ CDN列表查询成功: app_id={:?}, count={}, total={}",
            app_id,
            list.len(),
            total
        );
        Ok((list, total))
    }

    async fn delete(&self, id: i64) -> anyhow::Result<u64> {
        Ok(CdnDomainRepo::delete(&pg_pool(), id).await?)
    }
}

//////// END
