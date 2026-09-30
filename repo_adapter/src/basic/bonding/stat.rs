// repo_adapter/src/fs/file/stat.rs -- 适配器 - FS - 文件 - 统计
// 2026/8/14 14:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::fs::file::stat::FileStatPort;
use repository::cola_fs::pg::file::FileRepo;

////////

/// # [STAT ADAPTER] - 文件统计适配器
/// * `desc`: `通过 repo 统计文件相关数据`
#[derive(Debug, Default, Clone)]
pub struct FileStatAdapter;

#[async_trait]
impl FileStatPort for FileStatAdapter {
    ////////

    /// # 1. [ADAPTER] - 用户文件统计
    async fn stat_user_file_count(&self, uid: i64) -> Result<u64> {
        let count = FileRepo::stat_user_file_count(uid).await?;
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 用户文件计数统计: uid={}, count={}",
            uid,
            count
        );
        Ok(count as u64)
    }

    ////////

    /// # 2. [ADAPTER] - 用户存储容量统计
    async fn stat_user_storage_used(&self, uid: i64) -> Result<i64> {
        let used = FileRepo::stat_user_storage_used(uid).await?;
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 用户存储容量统计: uid={}, used={}bytes",
            uid,
            used
        );
        Ok(used)
    }
}

//////// END
