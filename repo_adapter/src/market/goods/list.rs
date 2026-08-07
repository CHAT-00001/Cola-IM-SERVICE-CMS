// repo_adapter/src/market/goods/list.rs
// 适配器 - 市场 - 商品 - 列表查询
// 2026/8/6 解耦: 最新/最热/推荐/同城/分类/搜索

////////

use anyhow::Result;
use cola_data::market::info::goods::goods::GoodsInfo;
use repository::market::pg::goods::GoodsRepo;

////////

/// # [ADAPTER] - 获取我的商品列表
pub async fn get_my_list(
    uid: i64, // 用户ID
    offset: i64, // 分页偏移
    limit: i64, // 每页数量
) -> Result<Vec<GoodsInfo>> {
    let entities = GoodsRepo::find_by_uid(uid, offset, limit).await?;
    Ok(entities.into_iter().map(GoodsInfo::from).collect())
}

/// # [ADAPTER] - 获取最新商品列表
pub async fn get_new_list(
    _uid: i64, // 用户ID
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<GoodsInfo>> {
    // 🚧 TODO: 对接 repository market service
    Ok(vec![])
}

/// # [ADAPTER] - 获取热门商品列表
pub async fn get_hot_list(
    _uid: i64, // 用户ID
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<GoodsInfo>> {
    // 🚧 TODO: 对接 repository market service
    Ok(vec![])
}

/// # [ADAPTER] - 获取推荐商品列表
pub async fn get_recommend_list(
    _uid: i64, // 用户ID
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<GoodsInfo>> {
    // 🚧 TODO: 对接 repository market service
    Ok(vec![])
}

/// # [ADAPTER] - 获取分类商品列表
pub async fn get_category_list(
    _uid: i64, // 用户ID
    _category_id: i64, // 分类ID
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<GoodsInfo>> {
    // 🚧 TODO: 对接 repository market service
    Ok(vec![])
}

//////// END