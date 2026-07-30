// cola_music/src/case/add.rs  -- MUSIC - 用例层 - 发布
// 2026-07-08 14:20

////////

use crate::assembler::music::build_music_single_response;
use anyhow::{Context, Result};
use cola_data::fs::rick_check;
use cola_data::music::command::music::MusicCommand;
use cola_data::music::vo::music_vo::MusicSingleResponse;
use repository::music::service::add::MusicAddService;
use tracing::info;

////////

/// # [CASE] - 音乐 发布 用例
pub struct MusicAddCase;

impl MusicAddCase {
    ////////

    /// # 1. [CASE] - 发布音乐
    pub async fn case_add_publish(
        uid: i64,
        cmd: MusicCommand,
    ) -> Result<MusicSingleResponse, anyhow::Error> {
        let check_text = format!("{} {}", cmd.name, cmd.description);

        let visibility = rick_check(check_text).await;

        let music_info = MusicAddService::save_music_and_update_count(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 音乐发布持久化失败: {}", e))?;

        info!("BIZ - 音乐发布成功: uid={}, visibility={}", uid, visibility);

        let response = build_music_single_response(music_info, Some(uid)).await?;

        Ok(response)
    }

    ////////

    /// # 2. [CASE] - 编辑音乐
    pub async fn case_edit_publish(
        uid: i64,
        cmd: MusicCommand,
    ) -> Result<MusicSingleResponse, anyhow::Error> {
        let check_text = format!("{} {}", cmd.name, cmd.description);

        let visibility = rick_check(check_text).await;

        let music_info = MusicAddService::edit_music(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 音乐编辑持久化失败: {}", e))?;

        info!("BIZ - 音乐编辑成功: uid={}, visibility={}", uid, visibility);

        let response = build_music_single_response(music_info, Some(uid)).await?;

        Ok(response)
    }
}

//////// END
