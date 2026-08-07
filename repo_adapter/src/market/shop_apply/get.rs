// repo_adapter/src/cola_market/shop_apply/get.rs
// 插头 - 市场 - 商店申请 - 获取对象状态/资料
// 2026/8/6 解耦: 获取商店申请详情、状态检查

////////

use anyhow::Result;

////////

/// # [ADAPTER] - 获取商店申请详情
pub async fn get_shop_apply_detail(
    _uid: i64, // 用户ID
    _shop_id: i64, // 店铺ID
) -> Result<()> {
    // 🚧 TODO: 对接 repository cola_market service
    Err(anyhow::anyhow!("not implemented"))
}

/// # [ADAPTER] - 获取商店状态
pub async fn get_shop_status(
    _uid: i64, // 用户ID
    _shop_id: i64, // 店铺ID
) -> Result<i16> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(0)
}

/// # [ADAPTER] - 获取用户已提交的申请列表
pub async fn get_user_apply_list(
    _uid: i64, // 用户ID
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<()>> {
    // 🚧 TODO: 对接 repository cola_market service
    Ok(vec![])
}

//////// END