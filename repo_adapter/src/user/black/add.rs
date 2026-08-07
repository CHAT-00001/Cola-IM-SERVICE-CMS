// repo_adapter/src/user/black/add.rs
// 🔌 适配器 - USER - 黑名单 - 发布
// 2026/8/6 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::user::port::black::add::BlackAddPort;
use repository::user::service::black::BlacklistService;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `用户黑名单发布插头`
pub struct BlackAddAdapter;

#[async_trait]
impl BlackAddPort for BlackAddAdapter {
    //

    ////////

    /// # [ADAPTER] - 添加黑名单
    async fn add_black(
        &self,
        uid: i64,       // 操作者ID
        target_id: i64, // 目标用户ID
    ) -> Result<()> {
        let _ = BlacklistService::save_black_record(uid, target_id, String::new(), 1).await?;
        Ok(())
    }

    ////////

    /// # [ADAPTER] - 移除黑名单
    async fn del_black(
        &self,
        uid: i64,       // 操作者ID
        target_id: i64, // 目标用户ID
    ) -> Result<()> {
        let _ = BlacklistService::save_black_record(uid, target_id, String::new(), 0).await?;
        Ok(())
    }
}

//////// END
