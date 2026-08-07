// repo_adapter/src/user/add/update.rs
// 适配器 - USER - Add - 更新用户
// 2026/8/6 解耦: 更新用户资料/头像/背景图

////////

use anyhow::Result;
use cola_data::user::command::user::update::UpdateUserCommand;
use cola_data::user::info::user::UserInfo;
use repository::user::pg::state_repo::UserStateRepo;

////////

/// # [ADAPTER] - 更新用户资料
/// * `desc`: 更新用户昵称/签名/邮箱等
pub async fn update_user(
    cmd: UpdateUserCommand, // 更新命令
) -> Result<UserInfo> {
    let entity = cmd.to_entity(0);
    let saved = UserStateRepo::save_user(entity)
        .await
        .map_err(|e| anyhow::anyhow!("[🤐 USER UPDATE ADAPTER]: ❌️ 更新用户失败: {}", e))?;

    Ok(saved.into())
}

////////

/// # [ADAPTER] - 更新头像
/// * `desc`: 更新用户头像URL
pub async fn update_avatar(
    _uid: i64, // 操作者ID
    _media_id: i64, // 媒体ID
) -> Result<()> {
    // 🚧 TODO: 对接 UserRepo 更新 avatar 字段
    Ok(())
}

////////

/// # [ADAPTER] - 更新背景图
/// * `desc`: 更新用户主页背景图URL
pub async fn update_bg(
    _uid: i64, // 操作者ID
    _media_id: i64, // 媒体ID
) -> Result<()> {
    // 🚧 TODO: 对接 UserRepo 更新 bg_img 字段
    Ok(())
}

//////// END