// port/src/fs/file/list.rs
// 🔌 端口 - FS - 文件 - 列表
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::entity::file::FsFileEntity;

////////

/// # [LIST PORT] - 文件列表
/// * `desc`: `FS - 文件列表端口`
#[async_trait::async_trait]
pub trait FileListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 用户文件列表
    /// * `desc`: `获取用户上传的文件列表`
    async fn list_user_files(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<FsFileEntity>>;

    ////////

    /// # 2. [PORT] - 特定应用的文件列表
    /// * `desc`: `获取某应用/模块的文件列表`
    async fn list_app_files(
        &self,
        app_id: String,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<FsFileEntity>>;
}

//////// END