// repo_adapter/src/cola_market/shop_apply/check.rs
// 插头 - 市场 - 商店申请 - 状态检查
// 2026/8/6 解耦: 检查是否已申请/是否可提交等校验

////////

use anyhow::Result;

////////

/// # [ADAPTER] - 检查用户是否已提交申请
/// * `desc`: 防止用户重复提交商店申请
pub async fn check_already_applied(
    _uid: i64, // 用户ID
) -> Result<bool> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(false)
}

////////

/// # [ADAPTER] - 检查商店名称是否可用
/// * `desc`: 商店名称唯一性校验
pub async fn check_shop_name_available(
    _name: &str, // 商店名称
) -> Result<bool> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(true)
}

////////

/// # [ADAPTER] - 检查用户是否有开店资格
/// * `desc`: 封禁用户/未实名用户不能开店
pub async fn check_user_can_open_shop(
    _uid: i64, // 用户ID
) -> Result<bool> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(true)
}

//////// END