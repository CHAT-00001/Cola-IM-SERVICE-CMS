// port/src/cola_video/buy/del.rs
// ⏩️ 端口 - ▶ 可乐视频 - 购买 - 删除
// 2026/8/5 01:52 Created.

////////

////////

/// # [DEL SERVICE] - 删除
/// `desc`: `视频购买删除端口`
#[async_trait::async_trait]
pub trait VideoBuyDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个删除
    async fn single_delete(
        &self,
        id: i64, // 目标 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量删除
    async fn batch_soft_del_record(
        &self,
        ids: Vec<i64>, // 目标 IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
