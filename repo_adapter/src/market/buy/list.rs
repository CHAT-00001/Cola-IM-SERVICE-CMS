// repo_adapter/src/video/buy/list.rs
// 🔌 适配器 - 商品 - 购买 - 列表 实现
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::buy::VideoBuyInfo;
use port::cola_video::buy::list::VideoBuyListPort;
use port::market::buy::list::GoodsBuyListPort;

////////

/// # [LIST ADAPTER] - 商品购买列表
/// * `desc`: 列表查询购买记录
#[derive(Debug, Default, Clone)]
pub struct GoodsBuyListAdapter;

#[async_trait]
impl GoodsBuyListPort for GoodsBuyListAdapter {
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

    /// # 2. [ADAPTER] - 商品的
    /// * `desc`: `根据商品ID` - `获取购买记录信息`
    async fn get_buy_infos_video_id(
        &self,
        uid: i64,
        video_id: i64, // 商品 ID
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoBuyInfo>)> {
        todo!()
    }
}

//////// END
