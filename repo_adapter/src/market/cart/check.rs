// repo_adapter/src/cola_video/cola_video/check.rs
// 🔌 插头 - MARKET - CART - 检查
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::cart::check::CartCheckPort;

////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `购物车检查适配器`
pub struct CartCheckAdapter;

#[async_trait]
impl CartCheckPort for CartCheckAdapter {
    async fn check_health(&self, video_id: i64) -> Result<(bool)> {
        todo!()
    }

    async fn check_state(&self, video_id: i64) -> Result<(bool)> {
        todo!()
    }

    async fn is_owner(&self, uid: i64, video_id: i64) -> Result<(bool)> {
        todo!()
    }
}

//////// END
