// port/src/fs/cdn/del.rs
// ⏩️ 端口 - FS - CDN - 删除
// 2026/8/5 00:03 Created.

////////

////////

/// # [DEL SERVICE] - 删除
/// `desc`: `视频评论删除服务端口`
#[async_trait::async_trait]
pub trait CdnDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个删除
    async fn single_delete(
        &self,
        id: i64, // 目标 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量删除
    async fn batch_delete(
        &self,
        ids: Vec<i64>, // 目标 IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
