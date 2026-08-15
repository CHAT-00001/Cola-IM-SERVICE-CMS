// port/src/fs/file/manage.rs
// 🔌 端口 - FS - 文件 - 管理
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::entity::file::FsFileEntity;

////////

/// # [MANAGE PORT] - 文件管理
/// * `desc`: `FS - 文件管理端口`
#[async_trait::async_trait]
pub trait FileManagePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 标记文件为正式（取消过期时间）
    /// * `desc`: `UGC 发布后调用，将临时文件转为正式文件`
    async fn mark_files_as_official(
        &self,
        uid: i64,
        file_ids: Vec<i64>,
        ref_table: String,
        ref_id: i64,
    ) -> Result<u64>;

    ////////

    /// # 2. [PORT] - 更新文件元数据
    async fn update_file_metadata(
        &self,
        file_id: i64,
        new_name: Option<String>,
        new_remark: Option<String>,
    ) -> Result<FsFileEntity>;

    ////////

    /// # 3. [PORT] - 更新文件状态
    async fn update_file_status(
        &self,
        file_id: i64,
        status: i16,
    ) -> Result<()>;
}

//////// END