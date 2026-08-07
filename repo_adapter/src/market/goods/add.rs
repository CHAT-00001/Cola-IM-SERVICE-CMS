// repo_adapter/src/cola_market/goods/add.rs
// 适配器 - 市场 - 商品 - 添加/更新
// 2026/8/6 解耦: 发布/编辑商品

////////

use anyhow::Result;
use chrono::Utc;
use cola_data::cola_market::command::goods::GoodsCommand;
use repository::cola_market::pg::goods::GoodsRepo;

////////

/// # [ADAPTER] - 保存商品
pub async fn save(
    uid: i64, // 用户ID
    cmd: GoodsCommand, // 商品命令
) -> Result<()> {
    cmd.validate()?;
    let mut entity = cmd.to_entity(uid);
    entity.uid = uid;
    entity.add_time = Utc::now().timestamp() as i32;
    entity.upd_time = entity.add_time;
    GoodsRepo::insert(&entity).await?;
    Ok(())
}

/// # [ADAPTER] - 编辑商品
pub async fn update(
    uid: i64, // 用户ID
    goods_id: i64, // 商品ID
    cmd: GoodsCommand, // 商品命令
) -> Result<()> {
    let mut entity = cmd.to_entity(uid);
    entity.uid = uid;
    entity.upd_time = Utc::now().timestamp() as i32;
    GoodsRepo::update(goods_id, &entity).await?;
    Ok(())
}

//////// END