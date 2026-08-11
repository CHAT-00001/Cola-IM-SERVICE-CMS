// repo_adapter/src/market/shop/check.rs
// 🔌 插头 - MARKET - 商店 - 检查
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::market::shop::check::ShopCheckPort;

////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `视频检查服务`
pub struct ShopCheckAdapter;

#[async_trait]
impl ShopCheckPort for ShopCheckAdapter {
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
