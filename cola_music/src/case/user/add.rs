// cola_music/src/case/user/add.rs  -- MUSIC - 用例层 - 用户统计数据 - 发布用例
// 2026/8/31 00:53 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::rick_check;
use cola_data::music::command::user::MusicUserCreateCommand;
use cola_data::music::info::user::MusicUserInfo; // 假设你的统计 Info 在这里
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [ADD CASE] - 音乐 用户统计数据发布用例
/// * `desc`: `可乐音乐 - 用户统计数据用例`
pub struct MusicUserAddCase;

impl MusicUserAddCase {
    //

    ////////

    /// # 1. [CASE] - 发布或更新统计数据
    pub async fn case_add_music_user(
        uid: i64,
        cmd: MusicUserCreateCommand,
        ctx: AppContext,
    ) -> Result<MusicUserInfo, anyhow::Error> {
        // 内容风控检查
        let check_text = format!("{} {}", cmd.name, cmd.description);
        let visibility = rick_check(check_text).await;

        // 调用下层服务，直接返回统计 Info
        let user_info = ctx
            .music
            .user
            .add
            .create_profile(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD CASE]: ❌️ 音乐用户统计数据持久化失败: {}", e))?;

        info!(
            "[🗣️ ADD CASE] - ✅️ 音乐用户统计数据更新成功: uid={}, visibility={}",
            uid, visibility
        );

        // 直接返回 Info，交由 API 层决定是包一层通用 AppData 还是直接吐出
        Ok(user_info)
    }

    ////////

    /// # 2. [CASE] - 编辑统计数据
    pub async fn case_edit_music_user(
        uid: i64,
        id: i64,
        cmd: MusicUserCreateCommand,
        ctx: AppContext,
    ) -> Result<MusicUserInfo, anyhow::Error> {
        let check_text = format!("{} {}", cmd.name, cmd.description);
        let visibility = rick_check(check_text).await;

        let user_info = ctx
            .music
            .user
            .add
            .update_profile(uid, id, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD CASE]: ❌️ 音乐用户统计数据修改持久化失败: {}", e))?;

        info!(
            "[🗣️ ADD CASE] - ✅️ 音乐用户统计数据编辑成功: uid={}, visibility={}",
            uid, visibility
        );

        Ok(user_info)
    }
}

//////// END