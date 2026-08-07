// repo_adapter/src/market/shop_apply/manage.rs
// 插头 - 市场 - 商店申请 - 管理操作
// 2026/8/6 解耦: 审核/驳回/封禁/改权限

////////

use anyhow::Result;
use cola_data::market::command::shop::add::CreatedShopApplyCommand;

////////

/// # [ADAPTER] - 审核通过商店申请
/// * `desc`: 管理员审核通过商店申请
pub async fn review_shop_apply(
    _uid: i64, // 管理员ID
    _shop_id: i64, // 店铺ID
    _cmd: CreatedShopApplyCommand, // 审核命令
) -> Result<()> {
    // 🚧 TODO: 对接 repository market service
    Err(anyhow::anyhow!("not implemented"))
}

////////

/// # [ADAPTER] - 驳回商店申请
/// * `desc`: 管理员驳回商店申请
pub async fn reject_shop_apply(
    _uid: i64, // 管理员ID
    _shop_id: i64, // 店铺ID
    _cmd: CreatedShopApplyCommand, // 驳回命令
) -> Result<()> {
    // 🚧 TODO: 对接 repository market service
    Err(anyhow::anyhow!("not implemented"))
}

////////

/// # [ADAPTER] - 封禁商店
/// * `desc`: 管理员封禁违规商店
pub async fn ban_shop(
    _uid: i64, // 管理员ID
    _shop_id: i64, // 店铺ID
    _reason: &str, // 封禁原因
) -> Result<()> {
    // 🚧 TODO: 对接 repository market service
    Err(anyhow::anyhow!("not implemented"))
}

////////

/// # [ADAPTER] - 修改商店权限
/// * `desc`: 管理员修改商店运营权限
pub async fn change_permission(
    _uid: i64, // 管理员ID
    _shop_id: i64, // 店铺ID
    _perm_id: i16, // 权限码
) -> Result<()> {
    // 🚧 TODO: 对接 repository market service
    Err(anyhow::anyhow!("not implemented"))
}

//////// END