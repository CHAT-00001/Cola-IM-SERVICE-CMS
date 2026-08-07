// repo_adapter/src/market/goods/del.rs
// 适配器 - 市场 - 商品 - 软删除
// 2026/8/6 解耦: 单个软删除 / 按用户批量删除

////////

use anyhow::Result;
use repository::market::pg::goods::GoodsRepo;

////////

/// # [ADAPTER] - 单个软删除商品
pub async fn soft_delete_single(
    _uid: i64, // 操作者ID
    goods_id: i64, // 商品ID
) -> Result<()> {
    GoodsRepo::delete(goods_id).await?;
    Ok(())
}

/// # [ADAPTER] - 按用户ID软删除所有商品
pub async fn soft_delete_by_user(
    _uid: i64, // 操作者ID
    _user_id: i64, // 目标用户ID
) -> Result<()> {
    // 🚧 TODO: 对接 repository market service
    Ok(())
}

//////// END