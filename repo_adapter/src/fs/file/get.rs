// repo_adapter/src/fs/file/get.rs
// 🔌 适配器 - FS - 文件 - 获取
// 2026/8/14 14:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::entity::file::FsFileEntity;
use port::fs::file::get::FileGetPort;
use repository::cola_fs::pg::file::FileRepo;

////////

/// # [GET ADAPTER] - 文件获取适配器
/// * `desc`: `通过 repo 查询文件，转成 info，回填缓存`
#[derive(Debug, Default, Clone)]
pub struct FileGetAdapter;

#[async_trait]
impl FileGetPort for FileGetAdapter {
    ////////

    /// # 1. [ADAPTER] - 按 ID 获取文件
    async fn get_file_by_id(&self, file_id: i64) -> Result<Option<FsFileEntity>> {
        // 1. 调用 repo 查询
        let file = FileRepo::get_file_by_id(file_id).await?;

        if let Some(ref entity) = file {
            // 2. Entity → Info（转换）
            let file_info = entity.to_file_info()?;

            // 3. 回填缓存
            // TODO: cache.set(&format!("file:{}", entity.id), &file_info).await?;

            tracing::info!("[🔌 ADAPTER] - ✅️ 文件查询成功: file_id={}", file_id);
        }

        Ok(file)
    }

    ////////

    /// # 2. [ADAPTER] - 按 Object Key 获取文件
    async fn get_file_by_object_key(&self, object_key: String) -> Result<Option<FsFileEntity>> {
        let file = FileRepo::get_file_by_object_key(&object_key).await?;

        if let Some(ref entity) = file {
            let file_info = entity.to_file_info()?;
            // TODO: cache.set(&format!("file:key:{}", object_key), &file_info).await?;
            tracing::info!("[🔌 ADAPTER] - ✅️ 文件查询成功: object_key={}", object_key);
        }

        Ok(file)
    }

    ////////

    /// # 3. [ADAPTER] - 批量按 ID 获取文件
    async fn batch_get_files(&self, file_ids: Vec<i64>) -> Result<Vec<FsFileEntity>> {
        let files = FileRepo::batch_get_files(file_ids).await?;

        for entity in &files {
            let file_info = entity.to_file_info()?;
            // TODO: cache.set(&format!("file:{}", entity.id), &file_info).await?;
        }

        tracing::info!("[🔌 ADAPTER] - ✅️ 批量查询文件成功: count={}", files.len());

        Ok(files)
    }
}
