// repo_adapter/src/fs/file/manage.rs
// 🔌 适配器 - FS - 文件 - 管理
// 2026/8/14 14:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::entity::file::FsFileEntity;
use port::fs::file::manage::FileManagePort;
use repository::cola_fs::pg::file::FileRepo;

////////

/// # [MANAGE ADAPTER] - 文件管理适配器
/// * `desc`: `通过 repo 更新文件，转成 info，回填缓存`
#[derive(Debug, Default, Clone)]
pub struct FileManageAdapter;

#[async_trait]
impl FileManagePort for FileManageAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 标记文件为正式（取消过期时间）
    /// * `desc`: `关键操作 - UGC 发布后调用，将临时文件转为正式`
    async fn mark_files_as_official(
        &self,
        uid: i64,
        file_ids: Vec<i64>,
        ref_table: String,
        ref_id: i64,
    ) -> Result<u64> {
        // 1. 调用 repo 更新文件状态
        let count = FileRepo::mark_files_as_official(
            uid,
            file_ids.clone(),
        )
        .await?;

        // 2. 清除缓存中所有这些文件的 info
        for file_id in file_ids {
            // TODO: cache.del(&format!("file:{}", file_id)).await?;
        }

        tracing::info!("[🔌 ADAPTER] - ✅️ 文件标记为正式成功: count={}", count);

        Ok(count)
    }

    ////////

    /// # 2. [ADAPTER] - 更新文件元数据
    async fn update_file_metadata(
        &self,
        file_id: i64,
        new_name: Option<String>,
        new_remark: Option<String>,
    ) -> Result<FsFileEntity> {
        let entity = FileRepo::update_file_metadata(
            file_id,
            new_name,
        )
        .await?;

        // 2. Entity → Info（转换）
        let file_info = entity.to_file_info()?;

        // 3. 回填缓存
        // TODO: cache.set(&format!("file:{}", entity.id), &file_info).await?;

        tracing::info!("[🔌 ADAPTER] - ✅️ 文件元数据更新成功: file_id={}", file_id);

        Ok(entity)
    }

    ////////

    /// # 3. [ADAPTER] - 更新文件状态
    async fn update_file_status(
        &self,
        file_id: i64,
        status: i16,
    ) -> Result<()> {
        FileRepo::update_file_status(file_id, status).await?;

        // 清除缓存
        // TODO: cache.del(&format!("file:{}", file_id)).await?;

        tracing::info!("[🔌 ADAPTER] - ✅️ 文件状态更新成功: file_id={}, status={}", file_id, status);

        Ok(())
    }
}

//////// END