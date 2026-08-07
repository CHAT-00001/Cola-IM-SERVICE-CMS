// repo_adapter/src/user/add/add.rs
// 适配器 - USER - Add - 添加用户
// 2026/8/6 解耦: 保存用户

////////

use anyhow::Result;
use cola_data::user::command::new::UserCommand;
use cola_data::user::entity::user::UserEntity;
use cola_data::user::info::user::UserInfo;
use repository::user::pg::state_repo::UserStateRepo;

////////

/// # [ADAPTER] - 保存用户资料
/// * `desc`: 创建新用户记录
pub async fn save_user(
    cmd: UserCommand, // 用户命令
) -> Result<UserInfo> {
    let entity = cmd.new();
    let saved = UserStateRepo::save_user(entity)
        .await
        .map_err(|e| anyhow::anyhow!("[🤐 USER ADD ADAPTER]: ❌️ 保存用户失败: {}", e))?;

    Ok(saved.into())
}

//////// END