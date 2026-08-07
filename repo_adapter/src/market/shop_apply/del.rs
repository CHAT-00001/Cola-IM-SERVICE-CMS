// repo_adapter/src/cola_market/shop_apply/del.rs
// 插头 - 市场 - 商店申请 - 软删除
// 2026/8/6 解耦: 单个软删除 / 批量软删除

////////

use anyhow::Result;

////////

/// # [ADAPTER] - 单个软删除商店申请
/// * `desc`: 管理员软删除单个商店
pub async fn soft_delete_single(
    _uid: i64, // 管理员ID
    _shop_id: i64, // 店铺ID
) -> Result<()> {
    // 🚧 TODO: 对接 repository cola_market service
    Err(anyhow::anyhow!("not implemented"))
}

////////

/// # [ADAPTER] - 批量软删除商店申请
/// * `desc`: 管理员批量软删除多个商店
pub async fn soft_delete_batch(
    _uid: i64, // 管理员ID
    _shop_ids: Vec<i64>, // 店铺ID列表
) -> Result<()> {
    // 🚧 TODO: 对接 repository cola_market service
    Err(anyhow::anyhow!("not implemented"))
}

//////// END