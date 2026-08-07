// repo_adapter/src/cola_market/shop_apply/list.rs
// 插头 - 市场 - 商店申请 - 列表查询
// 2026/8/6 解耦: 最新/最热/推荐/同城/分类/搜索

////////

use anyhow::Result;

////////

/// # [ADAPTER] - 获取最新申请列表
pub async fn get_new_list(
    _uid: i64, // 用户ID
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<()>> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(vec![])
}

/// # [ADAPTER] - 获取热门申请列表
pub async fn get_hot_list(
    _uid: i64, // 用户ID
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<()>> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(vec![])
}

/// # [ADAPTER] - 获取推荐申请列表
pub async fn get_recommend_list(
    _uid: i64, // 用户ID
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<()>> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(vec![])
}

/// # [ADAPTER] - 获取同城申请列表
pub async fn get_city_list(
    _uid: i64, // 用户ID
    _lat: f64, // 纬度
    _lng: f64, // 经度
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<()>> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(vec![])
}

/// # [ADAPTER] - 获取分类申请列表
pub async fn get_category_list(
    _uid: i64, // 用户ID
    _category_id: i64, // 分类ID
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<()>> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(vec![])
}

/// # [ADAPTER] - 搜索申请列表
pub async fn search_list(
    _uid: i64, // 用户ID
    _keyword: &str, // 搜索关键词
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<()>> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(vec![])
}

//////// END