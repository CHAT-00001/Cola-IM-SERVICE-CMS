// repo_adapter/src/fs/cdn/del.rs -- 适配器 - FS - CDN - 删除
// 2026/8/6 19:12 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::fs::cdn::del::CdnDelPort;

////////

/// # [DEL ADAPTER] - 发布
/// * `desc`: `FS - CDN逻辑删除适配器`
#[derive(Debug, Default, Clone)]
pub struct CdnDelAdapter;

#[async_trait]
impl CdnDelPort for CdnDelAdapter {
    async fn single_delete(&self, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END
