// repo_adapter/src/market/goods/get.rs
// 适配器 - 市场 - 商品 - 获取详情/状态
// 2026/8/6 解耦: 获取商品详情、状态检查

////////

use anyhow::Result;
use cola_data::market::info::goods::goods::GoodsInfo;
use repository::market::pg::goods::GoodsRepo;

////////

/// # [ADAPTER] - 浏览商品详情
pub async fn get_detail(
    _uid: i64, // 用户ID
    goods_id: i64, // 商品ID
) -> Result<GoodsInfo> {
    let entity = GoodsRepo::find_by_id(goods_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("商品不存在"))?;
    Ok(GoodsInfo::from(entity))
}

/// # [ADAPTER] - 获取商品状态
pub async fn get_status(
    _uid: i64, // 用户ID
    _goods_id: i64, // 商品ID
) -> Result<i16> {
    // 🚧 TODO: 对接 repository market service
    Ok(1)
}

//////// END