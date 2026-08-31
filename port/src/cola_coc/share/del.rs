// cola_video/port/share/del.rs
// 视频 - port - 分享 - 删除
// 2026/8/5 00:01 Created.

////////

////////

use std::arch::x86_64::CpuidResult;

/// # [DEL SERVICE] - 删除
/// `desc`: `视频分享删除服务端口`
#[async_trait::async_trait]
pub trait VideoShareDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个删除
    async fn delete_share_record(
        &self,
        uid: i64, // UID
        id: i64,  // 目标 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量删除
    async fn batch_delete(
        &self,
        ids: Vec<i64>, // 目标 IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
