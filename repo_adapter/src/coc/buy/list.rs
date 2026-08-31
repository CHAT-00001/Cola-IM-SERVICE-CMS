// repo_adapter/src/video/buy/list.rs
// 🔌 适配器 - 视频 - 购买 - 列表 实现
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::buy::VideoBuyInfo;
use port::cola_video::buy::list::VideoBuyListPort;

////////

/// # [LIST ADAPTER] - 视频购买列表
/// * `desc`: 列表查询购买记录
#[derive(Debug, Default, Clone)]
pub struct BuyListPortAdapter;

#[async_trait]
impl VideoBuyListPort for BuyListPortAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的
    /// * `desc`: `根据用户ID` - `获取购买记录信息`
    async fn get_buy_infos_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoBuyInfo>)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 视频的
    /// * `desc`: `根据视频ID` - `获取购买记录信息`
    async fn get_buy_infos_video_id(
        &self,
        uid: i64,
        video_id: i64, // 视频 ID
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoBuyInfo>)> {
        todo!()
    }
}

//////// END
