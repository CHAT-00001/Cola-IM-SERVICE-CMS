// repository/src/music/service/add.rs
// 仓储 - MUSIC - service - 发布（对齐视频发布模式）
// 2026-07-08
// 2026/8/4 重构：对齐 video/service/video/add.rs 原子化模式

////////

use crate::music::pg::music::basic::MusicRepo;
use anyhow::Result;
use cola_data::music::command::music::new::MusicCommand;
use cola_data::music::info::music::MusicInfo;
use tracing::log;

////////

/// # [ADD SERVICE] - 音乐 发布 服务
pub struct MusicAddService;

impl MusicAddService {
    //

    ////////

    /// # 1. [SERVICE] - 保存音乐 + 更新计数（对齐视频模式）
    pub async fn save_music_and_update_count(
        uid: i64,
        cmd: MusicCommand,
        visibility: i16,
    ) -> Result<MusicInfo, anyhow::Error> {
        // 🌟 返回值无缝升级为 MusicInfo
        // 1. Call Repo - 保存音乐并直接返回插入后的物理实体数据
        let music_entity = MusicRepo::save_music_by_uid(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("[🎵 SERVICE]: 写入音乐主表失败: {}", e))?;

        // 2. 🌟 核心升级：就地消化物理 Entity，转换为纯净领域元数据
        let music_info = MusicInfo::from_music_entity(&music_entity);

        Ok(music_info)
    }

    ////////

    /// # 2. [SERVICE] - 编辑音乐（复用保存逻辑）
    pub async fn edit_music(
        uid: i64,
        cmd: MusicCommand,
        visibility: i16,
    ) -> Result<MusicInfo, anyhow::Error> {
        MusicAddService::save_music_and_update_count(uid, cmd, visibility).await
    }
}


//////// END
