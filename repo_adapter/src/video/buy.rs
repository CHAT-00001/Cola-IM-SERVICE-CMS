// repo_adapter/src/video/buy.rs
// 2026-06-12 09:30
////////

use async_trait::async_trait;
use cola_data::video::port::buy::BuyRepo;
use repo::video::service::danmaku::DanmakuService;

////////

/// # [BUY PORT] - 购买 端口 插头
pub struct BuyPortAdapter;

////////

#[async_trait]
impl BuyRepo for BuyPortAdapter {

    ////////

    /// # 1. [PORT] - 保存购买记录 + 更新收买数量
    async fn save_buy_record(
        &self,
        uid: i64,
        video_id: i64,
    ) -> anyhow::Result<()> {
        DanmakuService::save_buy_and_update_count(uid, cola_data::video::command::buy::BuyCommand::default()).await
    }

    ////////


    /// # 2. [PORT] - 删除购买记录
    /// * `DESC`: 销量不可逆
    async fn del_buy_record(
        &self,
        _uid: i64,
        _video_id: i64,
    ) -> anyhow::Result<()> {
        // TODO: implement actual delete
        Ok(())
    }


    ////////

    /// # 3. [PORT] - 根据用户ID获取购买记录视频IDs
    async fn get_buy_ids_by_user_id(
        &self,
        _user_id: i64,
        _offset: i64,
        _limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        // TODO: implement actual query
        Ok(vec![])
    }
}

//////// END
