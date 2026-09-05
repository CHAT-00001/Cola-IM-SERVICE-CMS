// repo_adapter/src/user/share/add.rs  -- 适配器 - USER - 分享 - 发布适配器
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::share::add::UserShareAddPort;

////////

/// # [ADD ADAPTER] - 用户资料分享发布适配器
/// * `DESC`: `COLA USER - Share Add Adapter`
pub struct UserShareAddAdapter;

#[async_trait]
impl UserShareAddPort for UserShareAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 添加分享
    async fn add_share(
        &self,
        uid: i64, // 操作者 ID
        id: i64,  // 目标 ID
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 移除分享
    async fn del_share(
        &self,
        uid: i64, // 操作者 ID
        id: i64,  // 分享 ID
    ) -> Result<()> {
        todo!()
    }
}

//////// END
