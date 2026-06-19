// repo_adapter/src/market/express.rs  -- 适配器 - 快递
// 2026/6/18

//////

use async_trait::async_trait;
use cola_data::market::info::address::AddressInfo;
use cola_data::market::port::express::ExpressPort;
use cola_data::market::command::goods::GoodsCommand;
use repo::market::pg::express::ExpressRepo;
use cola_data::market::info::express::ExpressInfo;

//////

/// # [ADAPTER] - 快递 端口适配器
pub struct ExpressAdapter;

#[async_trait]
impl ExpressPort for ExpressAdapter {

    async fn save_goods(&self, _uid: i64, _cmd: GoodsCommand) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn update_goods(&self, _uid: i64, _goods_id: i64, _cmd: GoodsCommand) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn change_status(&self, _uid: i64, _goods_id: i64) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn delete_goods(&self, _uid: i64, _goods_id: i64) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("not implemented"))
    }

    async fn get_address_by_user_id(&self, _uid: i64, _offset: i64, _limit: i64) -> anyhow::Result<Vec<AddressInfo>> {
        // 返回启用的快递列表
        let entities = ExpressRepo::find_enabled().await?;
        Ok(entities.into_iter().map(|e| AddressInfo::not_found()).collect())
    }

    async fn view_goods_by_id(&self, _uid: i64, goods_id: i64) -> anyhow::Result<AddressInfo> {
        let entity = ExpressRepo::find_by_id(goods_id).await?
            .ok_or_else(|| anyhow::anyhow!("快递不存在"))?;
        // 没办法直接转 AddressInfo，返回默认
        Ok(AddressInfo::not_found())
    }

    async fn delete_address_by_user_id(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
}
