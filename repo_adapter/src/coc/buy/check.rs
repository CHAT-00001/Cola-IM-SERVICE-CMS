// repo_adapter/src/video/buy/checkt.rs
// 🔌 适配器 - ▶ 视频 - 购买 - 检查
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::buy::check::VideoBuyCheckPort;

////////

/// # [CHECK ADAPTER] - 视频购买检查
/// * `desc`: `▶ 视频 - 检查购买记录状态`
#[derive(Debug, Default, Clone)]
pub struct BuyCheckPortAdapter;

#[async_trait]
impl VideoBuyCheckPort for BuyCheckPortAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 检查购买状态
    /// * `desc`: 检查指定购买记录的状态
    async fn is_bought(
        &self,
        user_id: i64,  // 用户 ID
        video_id: i64, // 视频 ID
    ) -> Result<bool, anyhow::Error> {
        Ok(false)
    }
}

//////// END
