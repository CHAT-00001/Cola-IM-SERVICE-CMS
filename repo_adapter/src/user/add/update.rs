// repo_adapter/src/cola_user/add/update.rs
// 🔌 适配器 - 可乐用户 - 发布 - 更新用户
// 2026/8/6 解耦: 更新用户资料/头像/背景图

////////

use anyhow::Result;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use repository::cola_user::pg::user::add::UserAddRepo;

////////

/// # 1. [ADAPTER] - 更新用户资料
/// * `desc`: 更新用户昵称/签名/邮箱等
pub async fn update_user(
    cmd: UpdateUserCommand, // 更新命令
) -> Result<UserInfo> {
    let entity = cmd.to_entity(0);
    let saved = UserAddRepo::save_user(entity)
        .await
        .map_err(|e| anyhow::anyhow!("[🤐 USER UPDATE ADAPTER]: ❌️ 更新用户失败: {}", e))?;

    Ok(saved.into())
}

////////

/// # 2. [ADAPTER] - 更新头像
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