// port/src/fs/file/get.rs
// 🔌 端口 - FS - 文件 - 获取
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::entity::file::FsFileEntity;

////////

/// # [GET PORT] - 文件获取
/// * `desc`: `FS - 文件获取端口`
#[async_trait::async_trait]
pub trait FileGetPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 按 ID 获取文件
    async fn get_file_by_id(&self, file_id: i64) -> Result<Option<FsFileEntity>>;

    ////////

    /// # 2. [PORT] - 按 Object Key 获取文件
    async fn get_file_by_object_key(&self, object_key: String) -> Result<Option<FsFileEntity>>;

    ////////

    /// # 3. [PORT] - 批量按 ID 获取文件
    async fn batch_get_files(&self, file_ids: Vec<i64>) -> Result<Vec<FsFileEntity>>;
}

//////// END
