// repo_adapter/src/user/category/manage.rs -- 适配器 - USER - 分类 - 管理适配器
// 2026/8/10 04:16 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::category::manage::UserCategoryManagePort;

////////

/// # [MANAGE ADAPTER] - 用户分类管理适配器
/// * `DESC`: `COLA USER - Categories Manage Adapter`
pub struct CategoryManageAdapter;

#[async_trait]
impl UserCategoryManagePort for CategoryManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 配置
    async fn get_config(&self, _user_id: i64) -> anyhow::Result<(UserConfigInfo)> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 添加
    async fn add_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 移除
    async fn del_black(&self, _uid: i64, _id: i64) -> anyhow::Result<()> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 获取
    async fn get_following(&self, _uid: i64) -> anyhow::Result<(UserInfo)> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 删除
    async fn single_del(&self, _uid: i64, _id: i64) -> anyhow::Result<(u16)> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 批量删除
    async fn batch_del(&self, _uid: i64, _ids: Vec<i64>) -> anyhow::Result<(u16)> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 检查状态
    async fn check_state(&self, _uid: i64, _user_id: i64) -> anyhow::Result<(bool)> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 获取用户的黑名单
    async fn get_list_by_user_id(
        &self,
        _user_id: i64,
        _offset: i64,
        _limit: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 获取TA的黑名单
    async fn get_here_list(&self, _uid: i64, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        todo!()
    }
}

//////// END
