// repo_adapter/src/user/vip/del.rs -- 适配器 - USER - 贵宾 - 删除
// 2026/8/6 解耦: VIP删除操作

////////

use anyhow::Result;

////////

/// # [ADAPTER] - 单个删除
pub async fn single_del(
    _uid: i64,       // 操作者ID
    _target_id: i64, // 目标ID
) -> Result<u16> {
    Ok(0)
}

/// # [ADAPTER] - 批量删除
pub async fn batch_del(
    _uid: i64,      // 操作者ID
    _ids: Vec<i64>, // ID列表
) -> Result<u16> {
    Ok(0)
}

//////// END
