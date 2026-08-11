// repo_adapter/src/market/goods/add.rs
// 🔌 适配器 - MARKET - GOODS - 发布
// 2026/8/6 10:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use cola_data::cola_market::command::goods::GoodsCommand;
use port::market::goods::add::GoodsAddPort;
use repository::cola_market::pg::goods::add::GoodsAddRepo;

////////
/// # [ADD ADAPTER] - 商品 端口适配器
/// `desc`: `MARKET - 商品适配器`
pub struct GoodsAddAdapter;

#[async_trait]
impl GoodsAddPort for GoodsAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 保存商品
    async fn add_goods(&self, uid: i64, cmd: GoodsCommand) -> Result<()> {
        GoodsAddRepo::save_goods(cmd, 0).await;
        todo!()
    }

    async fn update_goods(&self, uid: i64, goods_id: i64, cmd: GoodsCommand) -> Result<()> {
        todo!()
    }

    async fn change_permission(&self, uid: i64, goods_id: i64, status_code: i16) -> Result<()> {
        todo!()
    }

    async fn change_status(&self, uid: i64, goods_id: i64, status_code: i16) -> Result<()> {
        todo!()
    }
}

//////// END
