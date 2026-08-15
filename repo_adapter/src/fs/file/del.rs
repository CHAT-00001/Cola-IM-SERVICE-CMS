// repo_adapter/src/fs/file/del.rs
// 🔌 适配器 - FS - 文件 - 删除
// 2026/8/14 14:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::fs::file::del::FileDelPort;
use repository::cola_fs::pg::file::FileRepo;

////////

/// # [DEL ADAPTER] - 文件删除适配器
/// * `desc`: `通过 repo 删除文件，清除缓存`
#[derive(Debug, Default, Clone)]
pub struct FileDelAdapter;

#[async_trait]
impl FileDelPort for FileDelAdapter {
    ////////

    /// # 1. [ADAPTER] - 单个删除文件
    async fn delete_file(
        &self,
        uid: i64,
        file_id: i64,
    ) -> Result<u64> {
        let count = FileRepo::delete_file(uid, file_id).await?;

        // 清除缓存
        // TODO: cache.del(&format!("file:{}", file_id)).await?;

        tracing::info!("[🔌 ADAPTER] - ✅️ 文件删除成功: file_id={}", file_id);

        Ok(count)
    }

    ////////

    /// # 2. [ADAPTER] - 批量删除文件
    async fn batch_delete_files(
        &self,
        uid: i64,
        file_ids: Vec<i64>,
    ) -> Result<u64> {
        let count = FileRepo::batch_delete_files(uid, file_ids.clone()).await?;

        // 清除所有缓存
        for file_id in file_ids {
            // TODO: cache.del(&format!("file:{}", file_id)).await?;
        }

        tracing::info!("[🔌 ADAPTER] - ✅️ 批量删除文件成功: count={}", count);

        Ok(count)
    }
}

//////// END
