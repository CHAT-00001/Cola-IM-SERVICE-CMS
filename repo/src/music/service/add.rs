// repo/src/music/service/add.rs  -- 仓储 - MUSIC - Service - 添加
// 2026-07-08

//////

use crate::music::pg::music::MusicRepo;
use crate::pg_pool;
use anyhow::Result;
use cola_data::music::command::music::MusicCommand;
use cola_data::music::info::music::MusicInfo;

//////

/// # [ADD SERVICE] - 音乐 添加 服务
pub struct MusicAddService;

impl MusicAddService {

    ////////

    /// # 1. [SERVICE] - 保存音乐 + 更新计数
    pub async fn save_music_and_update_count(
        uid: i64,
        cmd: MusicCommand,
        visibility: i16,
    ) -> Result<MusicInfo, anyhow::Error> {
        let entity = MusicRepo::save_music_by_uid(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 写入音乐主表失败: {}", e))?;

        // 从 entity 转换到 MusicInfo
        let id = entity.id as u64;
        let uuid = entity.uuid;
        let user_id = entity.author;
        let name = entity.title.unwrap_or_default();
        let cover_url = entity.cover_url;
        let duration = entity.duration.and_then(|d| d.parse::<u32>().ok()).unwrap_or(0);
        let href = entity.href;
        let description = entity.description;
        let add_time = entity.add_time.unwrap_or(0) as i64;

        let music_info = MusicInfo {
            id,
            uuid,
            user_id,
            actor: Some(format!("用户{}", uid)),
            name: name.clone(),
            name_en: None,
            cover_url,
            duration,
            release_time: String::new(),
            href,
            add_time,
            sync_time: add_time,
        };

        Ok(music_info)
    }

    ////////

    /// # 2. [SERVICE] - 编辑音乐
    pub async fn edit_music(
        uid: i64,
        cmd: MusicCommand,
        visibility: i16,
    ) -> Result<MusicInfo, anyhow::Error> {
        MusicAddService::save_music_and_update_count(uid, cmd, visibility).await
    }
}