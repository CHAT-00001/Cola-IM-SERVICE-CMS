// port/src/fs/file/add.rs
// 🔌 端口 - FS - 文件 - 新增
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::entity::file::FsFileEntity;

////////

/// # [ADD PORT] - 文件新增
/// * `desc`: `FS - 文件新增端口`
#[async_trait::async_trait]
pub trait FileAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 创建临时文件记录
    /// * `desc`: `创建 S3 文件映射记录（临时，支持自动过期）`
    async fn create_temp_file(
        &self,
        uid: i64,
        app_id: String,
        bucket: String,
        object_key: String,
        file_name: String,
        file_size: i64,
        mime_type: Option<String>,
        expires_in_days: i32,
    ) -> Result<FsFileEntity>;

    ////////

    /// # 2. [PORT] - 批量创建临时文件记录
    /// * `desc`: `批量创建 S3 文件映射`
    async fn batch_create_temp_files(
        &self,
        uid: i64,
        app_id: String,
        files: Vec<(String, String, String, i64, Option<String>)>,
        expires_in_days: i32,
    ) -> Result<Vec<FsFileEntity>>;
}

//////// END
