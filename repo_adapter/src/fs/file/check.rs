// repo_adapter/src/fs/file/check.rs
// 🔌 适配器 - FS - 文件 - 检查
// 2026/8/14 14:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::fs::file::check::FileCheckPort;
use repository::cola_fs::pg::file::FileRepo;

////////

/// # [CHECK ADAPTER] - 文件检查适配器
/// * `desc`: `通过 repo 检查文件状态`
#[derive(Debug, Default, Clone)]
pub struct FileCheckAdapter;

#[async_trait]
impl FileCheckPort for FileCheckAdapter {
    ////////

    /// # 1. [ADAPTER] - 检查文件存在
    async fn check_file_exists(&self, file_id: i64) -> Result<bool> {
        let exists = FileRepo::check_file_exists(file_id).await?;
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 文件存在检查: file_id={}, exists={}",
            file_id,
            exists
        );
        Ok(exists)
    }

    ////////

    /// # 2. [ADAPTER] - 检查文件可用
    async fn check_file_available(&self, file_id: i64) -> Result<bool> {
        let available = FileRepo::check_file_available(file_id).await?;
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 文件可用检查: file_id={}, available={}",
            file_id,
            available
        );
        Ok(available)
    }

    ////////

    /// # 3. [ADAPTER] - 检查文件所有权
    async fn check_file_owner(&self, uid: i64, file_id: i64) -> Result<bool> {
        let is_owner = FileRepo::check_file_owner(uid, file_id).await?;
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 文件所有权检查: uid={}, file_id={}, is_owner={}",
            uid,
            file_id,
            is_owner
        );
        Ok(is_owner)
    }
}

//////// END
