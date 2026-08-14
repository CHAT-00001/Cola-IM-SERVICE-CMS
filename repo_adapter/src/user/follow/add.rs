// repo_adapter/src/user/follow/add.rs
// 🔌 插头 - 可乐用户 - 关注 - 添加/取消
// 2026/8/6 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::follow::add::UserFollowAddPort;

////////

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `USER - 关注发布`
pub struct FollowAddAdapter;
#[async_trait]
impl UserFollowAddPort for FollowAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 添加关注
    async fn add_follow(&self, uid: i64, user_id: i64,) -> Result<()> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 移除关注
    async fn del_follow(&self, uid: i64, user_id: i64) -> Result<()> {
        todo!()
    }
}

//////// END
