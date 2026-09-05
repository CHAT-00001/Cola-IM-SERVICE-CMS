// repo_adapter/src/user/friend/add.rs -- 适配器 - USER - 朋友 - 发布适配器
// 2026/8/6 14:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::friend::add::FriendAddPort;

////////

/// # [ADD ADAPTER] - 用户朋友发布适配器
pub struct FriendAddAdapter;
#[async_trait]
impl FriendAddPort for FriendAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 添加朋友
    async fn create_friend(&self, uid: i64, user_id: i64) -> Result<()> {
        todo!()
    }

    async fn delete_friend(&self, uid: i64, user_id: i64) -> Result<()> {
        todo!()
    }

    async fn upsert_friend(&self, uid: i64, user_id: i64, status: i16) -> Result<()> {
        todo!()
    }

    async fn get_friending(&self, uid: i64) -> Result<(UserInfo)> {
        todo!()
    }

    async fn update_friend(&self, uid: i64, id: i64, remark: Option<String>) -> Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, uid: i64, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END
