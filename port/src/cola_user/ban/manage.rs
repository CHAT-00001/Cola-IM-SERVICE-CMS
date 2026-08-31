// port/src/user/ban/manage.rs
// ⏩️ 端口 - 🗣 用户 - 封禁 - 管理
// 2026/8/5 21:35 Created.

////////

use async_trait::async_trait;

////////

/// # [MANAGE PORTS]
/// * `desc`: `用户封禁申诉与工单管理端口`
#[async_trait]
pub trait UserBanManagePort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 列表
    /// * `desc`: `分页获取用户的封禁申诉工单列表`
    async fn get_appeal_list(
        &self,
        uid: i64,    // 操作者ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(i64, Vec<i64>)>; // 修正为返回总数与工单/用户IDs

    ////////

    /// # 2. [PORT] - 驳回
    /// * `desc`: `驳回用户的申诉请求`
    async fn set_deny_appeal(
        &self,
        uid: i64,           // 操作者ID
        user_ids: Vec<i64>, // 目标用户IDs
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 退回
    /// * `desc`: `申诉证据不足，需要补充资料`
    async fn set_reject_appeal(
        &self,
        uid: i64,           // 操作者ID
        user_ids: Vec<i64>, // 目标用户IDs
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 关闭
    /// * `desc`: `审核人员关闭申诉通道`
    async fn set_close_appeal(
        &self,
        uid: i64,           // 操作者ID
        user_ids: Vec<i64>, // 目标用户IDs
    ) -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - 切换/重新指派
    /// * `desc`: `审核人员切换或重新指派工单`
    async fn set_reassign_appeal(
        &self,
        uid: i64,           // 操作者ID
        user_ids: Vec<i64>, // 目标用户IDs
    ) -> anyhow::Result<()>;

    ////////

    /// # 6. [PORT] - 终审
    /// * `desc`: `审核人员进行申诉终审`
    async fn set_final_review(
        &self,
        uid: i64,           // 操作者ID
        user_ids: Vec<i64>, // 目标用户IDs
    ) -> anyhow::Result<()>;

    ////////

    /// # 7. [PORT] - 解封
    /// * `desc`: `通过申诉并解除用户封禁`
    async fn set_unban(
        &self,
        uid: i64,           // 操作者ID
        user_ids: Vec<i64>, // 目标用户IDs
    ) -> anyhow::Result<()>;
}

//////// END
