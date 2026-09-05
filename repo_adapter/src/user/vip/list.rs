// repo_adapter/src/user/vip/list.rs -- 适配器 - USER - 贵宾 - 列表适配器
// 2026/8/6 12:40 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::command::vip::VipCommand;
use cola_data::cola_user::info::vip::UserVipInfo;
use port::cola_user::view::add::UserViewAddPort;
use port::cola_user::vip::UserVipPort;
use port::cola_user::vip::add::VipAddPort;
use port::cola_user::vip::list::VipListPort;
use service::cola_user::vip::add::VipAddService;

////////

/// # [LIST ADAPTER] - 用户贵宾列表适配器
/// * `DESC`: `COLA USER - VIP List Adapter`
pub struct UserVipListAdapter;

#[async_trait]
impl VipListPort for UserVipListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 最新的
    async fn get_new_list(
        &self,
        uid: i64,    // 作业员
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<(Vec<UserVipInfo>)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 用户的
    async fn get_user_list(
        &self,
        uid: i64,     // 作业员
        user_id: i64, // 目标用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<(Vec<UserVipInfo>)> {
        todo!()
    }
}

//////// END
