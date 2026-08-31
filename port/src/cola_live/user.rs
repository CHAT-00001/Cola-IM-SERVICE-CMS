// port/src/cola_live/user.rs -- 端口 - LIVE - 用户资料 - mod
// 2026/8/20 Created.

////////




/// # [PORT] - 初始化直播域用户
/// * `desc`: `为全局用户创建直播扩展资料，等级从1开始，经验从0开始`
#[async_trait::async_trait]
pub trait LiveUserPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 初始化直播用户资料
    /// * `desc`: `幂等创建 cola_live.user`
    async fn init_live_user(&self, user_id: i64) -> anyhow::Result<()>;
}

//////// END
