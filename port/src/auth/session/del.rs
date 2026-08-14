// port/src/auth/session/del.rs
// ⏩️ 端口 - AUTH - 会话 - 删除
// 2026/8/5 00:03 Created.

////////

////////

/// # [DEL PORT] - 删除
/// `desc`: `AUTH- 会话端口`
#[async_trait::async_trait]
pub trait SessionDelPort: Send + Sync {
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
