// repo_adapter/src/cola_user/add/add.rs
// 🔌 适配器 - 可乐用户 - Add - 添加用户
// 2026/8/6 解耦: 保存用户

////////

use anyhow::Result;
use cola_data::cola_user::command::new::UserCommand;
use cola_data::cola_user::info::user::UserInfo;
use repository::cola_user::pg::user::add::UserAddRepo;

////////

/// # [ADAPTER] - 保存用户资料
/// * `desc`: 创建新用户记录
pub async fn save_user(cmd: UserCommand, // 用户命令
) -> Result<UserInfo> {
    // 1. Call ..
    let entity = cmd.new();

    // 2. Call ..
    let saved = UserAddRepo::save_user(entity)
        .await
        .map_err(|e| anyhow::anyhow!("[🤐 USER ADD ADAPTER]: ❌️ 保存用户失败: {}", e))?;

    Ok(saved.into())
}

//////// END
