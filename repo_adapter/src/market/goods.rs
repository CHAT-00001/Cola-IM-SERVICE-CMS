// repo_adapter/src/market/goods.rs  -- 适配器 - 商品
// 2026/6/18

////////

use async_trait::async_trait;
use chrono::Utc;
use cola_data::market::command::goods::GoodsCommand;
use cola_data::market::info::goods::GoodsInfo;
use cola_data::market::port::goods::GoodsPort;
use repository::market::pg::goods::GoodsRepo;

////////

/// # [ADAPTER] - 商品 端口适配器
pub struct GoodsAdapter;

#[async_trait]
impl GoodsPort for GoodsAdapter {

    async fn save_goods(&self, uid: i64, cmd: GoodsCommand) -> anyhow::Result<()> {
        cmd.validate()?;
        let mut entity = cmd.to_entity(uid);
        entity.uid = uid;
        entity.add_time = Utc::now().timestamp() as i32;
        entity.upd_time = entity.add_time;
        GoodsRepo::insert(&entity).await?;
        Ok(())
    }

    async fn update_goods(&self, uid: i64, goods_id: i64, cmd: GoodsCommand) -> anyhow::Result<()> {
        let mut entity = cmd.to_entity(uid);
        entity.uid = uid;
        entity.upd_time = Utc::now().timestamp() as i32;
        GoodsRepo::update(goods_id, &entity).await?;
        Ok(())
    }

    async fn change_status(&self, _uid: i64, goods_id: i64) -> anyhow::Result<()> {
        GoodsRepo::toggle_status(goods_id).await?;
        Ok(())
    }

    async fn delete_goods(&self, _uid: i64, goods_id: i64) -> anyhow::Result<()> {
        GoodsRepo::delete(goods_id).await?;
        Ok(())
    }

    async fn get_address_by_user_id(&self, uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<GoodsInfo>> {
        let entities = GoodsRepo::find_by_uid(uid, offset, limit).await?;
        Ok(entities.into_iter().map(GoodsInfo::from).collect())
    }

    async fn view_goods_by_id(&self, _uid: i64, goods_id: i64) -> anyhow::Result<GoodsInfo> {
        let entity = GoodsRepo::find_by_id(goods_id).await?
            .ok_or_else(|| anyhow::anyhow!("商品不存在"))?;
        Ok(GoodsInfo::from(entity))
    }

    async fn delete_address_by_user_id(&self, _uid: i64, _user_id: i64) -> anyhow::Result<()> {
        // 不需要实现
        Ok(())
    }
}
