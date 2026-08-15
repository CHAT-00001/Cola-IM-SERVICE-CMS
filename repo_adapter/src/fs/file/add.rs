// repo_adapter/src/fs/file/add.rs
// 🔌 适配器 - FS - 文件 - 新增
// 2026/8/14 14:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::entity::file::FsFileEntity;
use port::fs::file::add::FileAddPort;
use repository::cola_fs::pg::file::{FileRepo};

////////

/// # [ADD ADAPTER] - 文件新增适配器
/// * `desc`: `通过 repo 拿数据库初始数据，转成 info，回填缓存`
#[derive(Debug, Default, Clone)]
pub struct FileAddAdapter;

#[async_trait]
impl FileAddPort for FileAddAdapter {
    ////////

    /// # 1. [ADAPTER] - 创建临时文件记录
    /// * `desc`: `调用 repo 保存文件，转成 info，回填缓存`
    async fn create_temp_file(
        &self,
        _uid: i64,
        _app_id: String,
        _bucket: String,
        _object_key: String,
        _file_name: String,
        _file_size: i64,
        _mime_type: Option<String>,
        _expires_in_days: i32,
    ) -> Result<FsFileEntity> {
        // TODO: 实现临时文件创建逻辑
        todo!("create_temp_file not yet implemented")
    }

    ////////

    /// # 2. [ADAPTER] - 批量创建临时文件记录
    async fn batch_create_temp_files(
        &self,
        uid: i64,
        app_id: String,
        files: Vec<(String, String, String, i64, Option<String>)>,
        expires_in_days: i32,
    ) -> Result<Vec<FsFileEntity>> {
        let mut results = Vec::new();

        for (bucket, object_key, file_name, file_size, mime_type) in files {
            let entity = self
                .create_temp_file(
                    uid,
                    app_id.clone(),
                    bucket,
                    object_key,
                    file_name,
                    file_size,
                    mime_type,
                    expires_in_days,
                )
                .await?;
            results.push(entity);
        }

        tracing::info!("[🔌 ADAPTER] - ✅️ 批量创建临时文件成功: count={}", results.len());

        Ok(results)
    }
}

//////// END

