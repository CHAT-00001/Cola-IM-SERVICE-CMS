// port/src/fs/file/check.rs
// 🔌 端口 - FS - 文件 - 检查
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;

////////

/// # [CHECK PORT] - 文件检查
/// * `desc`: `FS - 文件检查端口`
#[async_trait::async_trait]
pub trait FileCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 检查文件存在
    /// * `desc`: `检查文件是否存在`
    async fn check_file_exists(
        &self,
        file_id: i64,
    ) -> Result<bool>;

    ////////

    /// # 2. [PORT] - 检查文件可用
    /// * `desc`: `检查文件状态是否正常`
    async fn check_file_available(
        &self,
        file_id: i64,
    ) -> Result<bool>;

    ////////

    /// # 3. [PORT] - 检查文件所有权
    /// * `desc`: `检查文件是否属于用户`
    async fn check_file_owner(
        &self,
        uid: i64,
        file_id: i64,
    ) -> Result<bool>;
}

//////// END
