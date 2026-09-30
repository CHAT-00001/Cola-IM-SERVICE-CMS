// port/src/fs/file/del.rs
// 🔌 端口 - FS - 文件 - 删除
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;

////////

/// # [DEL PORT] - 文件删除
/// * `desc`: `FS - 文件逻辑删除端口`
#[async_trait::async_trait]
pub trait FileDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个删除文件
    async fn delete_file(&self, uid: i64, file_id: i64) -> Result<u64>;

    ////////

    /// # 2. [PORT] - 批量删除文件
    async fn batch_delete_files(&self, uid: i64, file_ids: Vec<i64>) -> Result<u64>;
}

//////// END
