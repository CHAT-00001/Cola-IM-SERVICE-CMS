// repo_adapter/src/fs/file/list.rs
// 🔌 适配器 - FS - 文件 - 列表
// 2026/8/14 14:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::entity::file::FsFileEntity;
use port::fs::file::list::FileListPort;
use repository::cola_fs::pg::file::FileRepo;

////////

/// # [LIST ADAPTER] - 文件列表适配器
/// * `desc`: `通过 repo 查询文件列表，转成 info，回填缓存`
#[derive(Debug, Default, Clone)]
pub struct FileListAdapter;

#[async_trait]
impl FileListPort for FileListAdapter {
    ////////

    /// # 1. [ADAPTER] - 用户文件列表
    async fn list_user_files(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<FsFileEntity>> {
        let files = FileRepo::list_user_files(uid, limit, offset).await?;

        for entity in &files {
            let file_info = entity.to_file_info()?;
            // TODO: cache.set(&format!("file:{}", entity.id), &file_info).await?;
        }

        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 用户文件列表查询成功: uid={}, count={}",
            uid,
            files.len()
        );

        Ok(files)
    }

    ////////

    /// # 2. [ADAPTER] - 特定应用的文件列表
    async fn list_app_files(
        &self,
        app_id: String,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<FsFileEntity>> {
        let files = FileRepo::list_app_files(&app_id, limit, offset).await?;

        for entity in &files {
            let file_info = entity.to_file_info()?;
            // TODO: cache.set(&format!("file:{}", entity.id), &file_info).await?;
        }

        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 应用文件列表查询成功: app_id={}, count={}",
            app_id,
            files.len()
        );

        Ok(files)
    }
}
