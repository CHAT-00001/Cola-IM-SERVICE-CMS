// repo_adapter/src/video/buy/stat.rs
// 🔌 适配器 - ▶ 视频 - 购买记录 - 统计
// 2026/8/8 10:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::buy::stat::VideoBuyStatPort;

////////

/// # [STAT ADAPTER] - 统计
/// * `desc`: `▶ 视频 - 购买记录统计查询`
#[derive(Debug, Default, Clone)]
pub struct BuyStatPortAdapter;

#[async_trait]
impl VideoBuyStatPort for BuyStatPortAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 获取用户购买数量
    /// * `desc`: `根据用户ID` - `统计用户购买的视频总数`
    async fn stat_count_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
    ) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 1. [ADAPTER] - 获取用户购买数量
    /// * `desc`: `根据用户ID` - `统计用户购买的视频总数`
    async fn stat_count_by_video_id(
        &self,
        uid: i64,
        video_id: i64, // 视频 ID
    ) -> Result<(u64)> {
        todo!()
    }
}

//////// END
