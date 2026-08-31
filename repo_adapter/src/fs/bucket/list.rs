// repo_adapter/src/fs/bucket/list.rs
// 🔌 适配器 - FS - 存储桶 -评论列表
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::info::bucket::BucketInfo;
use port::fs::bucket::list::BucketListPort;
use repository::cola_fs::pg::bucket::BucketRepo;
use repository::pg_pool;

////////

/// # [LIAT ADAPTER] - 列表
/// * `desc`: `FS - 存储桶列表适配器`
#[derive(Debug, Default, Clone)]
pub struct BucketListAdapter;

#[async_trait]
impl BucketListPort for BucketListAdapter {
    async fn admin_find_page(
        &self,
        app_id: Option<&str>,
        keyword: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<BucketInfo>, i64)> {
        let (entities, total) =
            BucketRepo::admin_find_page(&pg_pool(), app_id, keyword, limit, offset).await?;
        let list: Vec<BucketInfo> = entities.into_iter().map(Into::into).collect();
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 管理员存储桶列表查询成功: app_id={:?}, count={}, total={}",
            app_id,
            list.len(),
            total
        );
        Ok((list, total))
    }
}
