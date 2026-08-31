// cola_music/src/case/music/add.rs  -- MUSIC - 用例层 - 音乐 - 发布用例
// 2026-07-08 14:20

////////

use crate::assembler::music::build_music_single_response;
use anyhow::Result;
use cola_data::cola_fs::rick_check;
use cola_data::music::command::music::new::{MusicCreateCommand, MusicUpdateCommand};
use cola_data::music::vo::music::MusicSingleResponse;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 音乐 发布 用例
/// * `desc`: `可乐音乐 - 发布用例`
pub struct MusicAddCase;

impl MusicAddCase {
    //

    ////////

    /// # 1. [CASE] - 发布音乐
    pub async fn case_add_music(
        uid: i64,
        cmd: MusicCreateCommand,
        ctx: AppContext,
    ) -> Result<MusicSingleResponse, anyhow::Error> {
        // 内容风控检查
        let check_text = format!("{} {}", cmd.name, cmd.description);

        let visibility = rick_check(check_text).await;

        // Call CTX ..
        let music_info = ctx
            .music
            .music
            .add
            .create_music(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD CASE]: ❌️ 音乐发布持久化失败: {}", e))?;

        info!(
            "[🗣️ ADD CASE] - ✅️ 音乐发布成功: uid={}, visibility={}",
            uid, visibility
        );

        let response = build_music_single_response(music_info, Some(uid)).await?;

        Ok(response)
    }

    ////////

    /// # 2. [CASE] - 编辑音乐
    pub async fn case_edit_music(
        uid: i64,
        id: i64,                 // 音乐 ID
        cmd: MusicUpdateCommand, // 更新命令
        ctx: AppContext,         // 应用上下文
    ) -> Result<MusicSingleResponse, anyhow::Error> {
        let check_text = format!("{} {}", cmd.name, cmd.description);

        let visibility = rick_check(check_text).await;

        // Call CTX ..
        let music_info = ctx
            .music
            .music
            .add
            .update_music(uid, id, cmd, visibility).await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD CASE]: ❌️ 音乐修改持久化失败: {}", e))?;

        info!(
            "[🗣️ ADD CASE] - ✅️ 音乐编辑成功: uid={}, visibility={}",
            uid, visibility
        );

        let response = build_music_single_response(music_info, Some(uid)).await?;

        Ok(response)
    }
}

//////// END
