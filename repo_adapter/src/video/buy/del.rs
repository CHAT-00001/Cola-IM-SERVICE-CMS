// repo_adapter/src/video/buy/del.rs
// 🔌 适配器 -  视频 - 购买 - 删除
// 2026/8/8 14:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::buy::del::VideoBuyDelPort;
use tracing::info;

////////

/// # [DELETE ADAPTER] - 视频购买删除
/// * `desc`: `视频 - 删除购买记录服务`
#[derive(Debug, Default, Clone)]
pub struct BuyDelPortAdapter;

#[async_trait]
impl VideoBuyDelPort for BuyDelPortAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 单个删除
    /// * `desc`: `逻辑删除购买记录`
    async fn single_soft_del_record(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
        id: i64,       // 目标 ID
    ) -> Result<(u16)> {
        let count = 1;

        info!("[🗣️ DELETE ADAPTER]: - ✅️ 删除购买记录成功! 共 {} 条.", count);
        todo!()
    }

    ////////

    /// # 2. [SERVICE] - 批量删除
    /// * `desc`: `逻辑删除购买记录`
    async fn batch_soft_del_record(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
        ids: Vec<i64>, // 目标 IDs
    ) -> Result<(u16)> {
        let count = 10;

        info!("[🗣️ DELETE ADAPTER]: - ✅️ 删除购买记录成功! 共 {} 条.", count);
        todo!()
    }
}

//////// END
