// repo_adapter/src/cola_market/goods/manage.rs
// 适配器 - 市场 - 商品 - 管理操作
// 2026/8/6 解耦: 上架/下架/封禁/改权限

////////

use anyhow::Result;
use repository::cola_market::pg::goods::GoodsRepo;

////////

/// # [ADAPTER] - 修改商品状态(上架/下架)
pub async fn change_status(
    _uid: i64, // 操作者ID
    goods_id: i64, // 商品ID
) -> Result<()> {
    GoodsRepo::toggle_status(goods_id).await?;
    Ok(())
}

/// # [ADAPTER] - 封禁商品
pub async fn ban_goods(
    _uid: i64, // 管理员ID
    _goods_id: i64, // 商品ID
    _reason: &str, // 封禁原因
) -> Result<()> {
    // 🚧 TODO: 对接 repository cola_market service
    Err(anyhow::anyhow!("not implemented"))
}

/// # [ADAPTER] - 修改商品权限
pub async fn change_permission(
    _uid: i64, // 管理员ID
    _goods_id: i64, // 商品ID
    _perm_id: i16, // 权限码
) -> Result<()> {
    // 🚧 TODO: 对接 repository cola_market service
    Err(anyhow::anyhow!("not implemented"))
}

//////// END