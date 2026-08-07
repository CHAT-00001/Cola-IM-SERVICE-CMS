// repo_adapter/src/cola_market/goods/check.rs
// 适配器 - 市场 - 商品 - 状态检查
// 2026/8/6 解耦: 检查商品是否存在/是否可上架等校验

////////

use anyhow::Result;

////////

/// # [ADAPTER] - 检查商品是否存在
pub async fn check_exists(
    _uid: i64, // 用户ID
    _goods_id: i64, // 商品ID
) -> Result<bool> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(true)
}

/// # [ADAPTER] - 检查商品名称是否可用
pub async fn check_name_available(
    _name: &str, // 商品名称
) -> Result<bool> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(true)
}

/// # [ADAPTER] - 检查用户是否有发布资格
pub async fn check_user_can_publish(
    _uid: i64, // 用户ID
) -> Result<bool> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(true)
}

//////// END