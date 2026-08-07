// repo_adapter/src/cola_user/black/manage.rs
// 适配器 - USER - 黑名单 - 管理操作
// 2026/8/6 解耦: 管理操作接口

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::info::black::UserBlackInfo;
use cola_data::cola_user::port::black::manage::BlackManagePort;

////////

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `管理员封禁服务适配器`
pub struct BlackManageAdapter;

// 构造实现
#[async_trait]
impl BlackManagePort for BlackManageAdapter {
    //

    ////////

    /// # [EMPTY SERVICE] - 空服务
    /// `warning`: `⚠️ 因为黑名单是私密关系,没有管理员接口`
    async fn get_black_list(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<UserBlackInfo>)> {
        todo!()
    }
}

//////// END
