// repo_adapter/src/fs/bucket/del.rs
// 🔌 适配器 - FS - 存储桶 -逻辑删除
// 2026/8/6 19:12 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::basic::app::del::AppDelPort;

////////

/// # [DEL ADAPTER] - 发布
/// * `desc`: `FS - 存储桶逻辑删除适配器`
#[derive(Debug, Default, Clone)]
pub struct AppDelAdapter;

#[async_trait]
impl AppDelPort for AppDelAdapter {
    async fn single_delete(&self, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END
