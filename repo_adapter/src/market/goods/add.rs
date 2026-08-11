// repo_adapter/src/cola_market/goods/add.rs
// 🔌 适配器 - MARKET - 商品 - 发布
// 2026/8/6 解耦: 发布/编辑商品

////////

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use cola_data::cola_market::command::goods::GoodsCommand;
use cola_data::cola_market::info::goods::goods::GoodsInfo;
use port::cola_market::goods::add::GoodsAddPort;
use repository::cola_market::pg::goods::add::GoodsAddRepo;
use repository::cola_market::pg::goods::GoodsRepo;

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
        GoodsAddRepo::save_goods(cmd).await;
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
