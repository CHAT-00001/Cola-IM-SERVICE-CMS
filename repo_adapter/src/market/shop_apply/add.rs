// repo_adapter/src/market/shop_apply/add.rs
// 插头 - 市场 - 商店申请 - 添加/更新
// 2026/8/6 解耦: 发布/编辑商店申请

////////

use anyhow::Result;
use cola_data::market::command::shop::add::CreatedShopApplyCommand;

////////

/// # [ADAPTER] - 保存商店申请
/// * `desc`: 保存用户提交的商店申请，并发送事件通知
pub async fn save_shop_apply(
    _uid: i64, // 用户ID
    _cmd: CreatedShopApplyCommand, // 申请命令
) -> Result<()> {
    // 🚧 TODO: 对接 repository market service
    Err(anyhow::anyhow!("not implemented"))
}

/// # [ADAPTER] - 编辑商店申请
/// * `desc`: 更新用户已有的商店申请，并发送事件通知
pub async fn update_shop_apply(
    _uid: i64, // 用户ID
    _shop_id: i64, // 店铺ID
    _cmd: CreatedShopApplyCommand, // 申请命令
) -> Result<()> {
    // 🚧 TODO: 对接 repository market service
    Err(anyhow::anyhow!("not implemented"))
}

//////// END