// repo_adapter/src/user/vip/get.rs
// 🔌 插头 - 可乐用户 - 贵宾 - 获取
// 2026/8/6 Created.

////////

use anyhow::Result;

////////

/// # [ADAPTER] - 获取VIP IDs
pub async fn get_vip_ids(
    _uid: i64, // 用户ID
    _offset: i64, // 分页偏移
    _limit: i64, // 每页数量
) -> Result<Vec<i64>> {
    // 🚧 TODO: 对接 VipService
    Ok(vec![])
}

//////// END