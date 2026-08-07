// repo_adapter/src/cola_user/add/del.rs
// 适配器 - USER - Add - 删除用户
// 2026/8/6 解耦: 单个删除/批量删除

////////

use anyhow::Result;

////////

/// # [ADAPTER] - 删除单个用户
pub async fn del_one_user(
    _user_id: i64, // 用户ID
) -> Result<()> {
    // 🚧 TODO: 对接 UserRepo 软删除
    Ok(())
}

////////

/// # [ADAPTER] - 批量删除用户
pub async fn del_many_user(
    _user_ids: Vec<i64>, // 用户ID列表
) -> Result<()> {
    // 🚧 TODO: 对接 UserRepo 批量软删除
    Ok(())
}

//////// END