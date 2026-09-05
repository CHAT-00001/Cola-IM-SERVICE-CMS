// repo_adapter/src/user/share/list.rs  -- 适配器 - USER - 分享 - 列表适配器
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::info::share::UserShareInfo;
use port::cola_user::share::list::UserShareListPort;

////////

pub struct ShareListAdapter;

#[async_trait]
impl UserShareListPort for ShareListAdapter {
    /// # 1. [ADAPTER] - 用户分享的
    async fn get_share_infos_by_user_id(
        &self,
        user_id: i64, // 用户ID
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<UserShareInfo>)> {
        todo!()
    }

    /// # 2. [ADAPTER] - 主页的分享
    async fn get_share_infos_by_profile_id(
        &self,
        profile_id: i64, // 主页 ID
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<UserShareInfo>)> {
        todo!()
    }
}

//////// END
