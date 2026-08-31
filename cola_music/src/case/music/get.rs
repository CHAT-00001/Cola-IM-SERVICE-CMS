// cola_music/src/case/music/get.rs -- MUSIC - 用例层 - 音乐 - 获取用例
// 2026/8/25 01:03 Created.

////////

use crate::assembler::music::build_music_single_response;
use anyhow::Result;
use cola_data::cola_fs::rick_check;
use cola_data::music::command::music::new::{MusicCreateCommand, MusicUpdateCommand};
use cola_data::music::vo::music::MusicSingleResponse;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [GET CASE] - 音乐内容获取用例
/// * `desc`: `可乐音乐 - 内容获取用例`
pub struct MusicGetCase;

impl MusicGetCase {
    //

    ////////

    /// # 1. [CASE] - 单个音乐信息
    pub async fn case_get_info(
        uid: i64,
        music_id: i64, // 音乐 ID
        ctx: AppContext,
    ) -> Result<MusicSingleResponse, anyhow::Error> {
        //


        // Call CTX ..
        let music_info = ctx
            .music
            .music
            .get
            .get_music_info(music_id)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 GET CASE]: ❌️ 单个获取音乐信息失败: {}", e))?;

        info!(
            "[🗣️ GET CASE] - ✅️ 单个获取音乐信息成功: id={}, music_id={}",
            uid, music_id
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
            .update_music(uid, id, cmd, visibility)
            .await
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
