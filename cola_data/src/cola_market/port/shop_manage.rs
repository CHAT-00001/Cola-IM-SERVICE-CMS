// port/shop_manage.rs  -- 服务端口 - 店铺申请管理
// 2026/6/18 12:34

////////

use crate::cola_market::vo::shop_apply::ShopApplyVo;
use crate::cola_market::vo::shop_apply_history::ShopApplyHistoryVo;

////////

/// # [SERVICE PORT] - 商店管理 服务端口
#[async_trait::async_trait]
pub trait ShopManagePort: Send + Sync {

    ////////

    /// # 1. [PORT] - 获取申请列表
    async fn get_apply(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(ShopApplyVo)>;

    ////////

    /// # 2. [PORT] - 获取申请历史记录
    async fn get_apply_history(
        &self,
        uid: i64,
        shop_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(ShopApplyHistoryVo)>;


    ////////

    /// # 3. [PORT] - 审核申请
    /// * `desc` 包含通过/驳回 + 原因
    async fn review_apply(
        &self,
        uid: i64,
        shop_id: i64,
        reason: String
    ) -> anyhow::Result<()>;


    ////////

    /// # 4. [PORT] - 强制关闭申请
    /// * `desc` 在审核阶段直接废弃该申请
    async fn abort_apply(
        &self,
        uid: i64,
        shop_id: i64,
        reason: String,
    ) -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - 修改商店运营状态
    /// * `desc` 例如：正常营业、暂时停业、永久封禁
    async fn change_status(
        &self,
        uid: i64,
        shop_id: i64,
        status: i16,
    ) -> anyhow::Result<()>;

    ////////

    /// # 6. [PORT] - 管理员批量删除多个商店
    /// * `desc` 软删除批量删除一批店铺
    async fn batch_delete(
        &self,
        uid: i64,
        shop_ids: Vec<i64>,
    ) -> anyhow::Result<()>;
}

//////// END