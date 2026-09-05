// repo_adapter/src/user/friend/check.rs -- 适配器 - USER - 朋友 - 发布适配器
// 2026/8/10 04:17 Created.

////////

use async_trait::async_trait;
use port::cola_user::friend::check::FriendCheckPort;
////////

/// # [CHECK ADAPTER] - 检查
/// * `DESC`: `COLA USER - Friend Check Adapter`
pub struct FriendCheckAdapter;

#[async_trait]
impl FriendCheckPort for FriendCheckAdapter {
    //

    ////////

    /// 1. # [ADAPTER] - 是朋友
    async fn is_friended(&self, uid: i64, id: i64) -> anyhow::Result<(bool)> {
        todo!()
    }
}

//////// END
