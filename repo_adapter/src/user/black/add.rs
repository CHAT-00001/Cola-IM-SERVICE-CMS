// repo_adapter/src/cola_user/black/add.rs
// 🔌 适配器 - USER - 黑名单 - 发布
// 2026/8/6 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::port::black::add::BlackAddPort;
use repository::cola_user::pg::black::add::UserBlackAddRepo;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `用户黑名单发布插头`
pub struct BlackAddAdapter;

#[async_trait]
impl BlackAddPort for BlackAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 添加黑名单
    async fn add_black(
        &self,
        uid: i64,       // 操作者ID
        target_id: i64, // 目标用户ID
    ) -> Result<()> {
        let _ = UserBlackAddRepo::save_add_black(uid, target_id, String::new(), 1).await?;
        Ok(())
    }

    ////////

    /// # 2. [ADAPTER] - 移除黑名单
    async fn del_black(
        &self,
        uid: i64,       // 操作者ID
        target_id: i64, // 目标用户ID
    ) -> Result<()> {
        let _ = UserBlackAddRepo::update_unblock_black(uid, target_id).await?;
        Ok(())
    }
}

//////// END
