// repo_adapter/src/marketo/express/add.rs
// 🔌 适配器 - MARKET - 快递公司 - 发布
// 2026-06-12 10:52 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::market::command::goods::GoodsCommand;
use cola_data::market::info::express::express::ExpressInfo;
use port::market::express::add::ExpressAddPort;


////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `MARKET - 快递公司发布适配器`
pub struct ExpressAddAdapter;

#[async_trait]
impl ExpressAddPort for ExpressAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 发布新视频
    async fn save_express(&self, uid: i64, cmd: GoodsCommand) -> Result<(ExpressInfo)> {
        todo!()
    }

    async fn update_express(&self, uid: i64, express_id: i64, cmd: GoodsCommand) -> Result<(ExpressInfo)> {
        todo!()
    }

    async fn change_status(&self, express_id: i64, status_code: i16) -> Result<()> {
        todo!()
    }

}

//////// END
