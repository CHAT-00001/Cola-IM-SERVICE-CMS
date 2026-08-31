// repo_adapter/src/video/buy/get.rs
// 🔌 适配器 - 视频 - 购买 - 获取 服务
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::buy::get::VideoBuyGetPort;
use repository::video::pg::buy::get::VideoBuyGetRepo;

////////

/// # [GET ADAPTER] - 视频购买获取
/// * `desc`: `▶ 可乐视频 - 获取购买记录`
#[derive(Debug, Default, Clone)]
pub struct BuyGetPortAdapter;

#[async_trait]
impl VideoBuyGetPort for BuyGetPortAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 获取我的购买记录的视频IDs
    /// * `desc`: 根据buy_id获取单个购买记录详情
    async fn get_my_bought_ids(
        &self,
        uid: i64,     // 操作者 ID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<(Vec<i64>)> {
        todo!()
    }

    ////////

    /// # 2. [SERVICE] - 获取购买记录的视频IDs
    /// * `desc`: 根据buy_id获取单个购买记录详情
    async fn get_he_bought_ids(
        &self,
        uid: i64,     // 操作者 ID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)> {
        // Call REPOSITORY ..
        let video_ids = VideoBuyGetRepo::find_video_ids_by_user_id(user_id, limit, offset).await?;

        Ok(video_ids)
    }
}

//////// END
