// port/src/fs/file/stat.rs
// 🔌 端口 - FS - 文件 - 统计
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;

////////

/// # [STAT PORT] - 文件统计
/// * `desc`: `FS - 文件统计端口`
#[async_trait::async_trait]
pub trait FileStatPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 用户文件统计
    /// * `desc`: `统计用户上传的文件总数`
    async fn stat_user_file_count(
        &self,
        uid: i64,
    ) -> Result<u64>;

    ////////

    /// # 2. [PORT] - 用户存储容量统计
    /// * `desc`: `统计用户已用存储容量（字节）`
    async fn stat_user_storage_used(
        &self,
        uid: i64,
    ) -> Result<i64>;
}

//////// END
