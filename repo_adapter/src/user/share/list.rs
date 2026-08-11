// repo_adapter/src/user/share/list.rs
// 🔌 插头 - 可乐用户 - 分享 - 列表
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::info::black::UserBlackInfo;
use cola_data::cola_user::info::share::ShareInfo;
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
    ) -> Result<(Vec<ShareInfo>)> {
        todo!()
    }

    /// # 2. [ADAPTER] - 主页的分享
    async fn get_share_infos_by_profile_id(
        &self,
        profile_id: i64,  // 主页 ID
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<ShareInfo>)> {
        todo!()
    }
}

//////// END
