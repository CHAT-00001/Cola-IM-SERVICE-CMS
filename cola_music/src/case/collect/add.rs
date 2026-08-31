// cola_music/src/case/collect/add.rs  -- MUSIC - 用例层 - 收藏 - 发布用例
// 2026-07-08 14:20

////////

use anyhow::{Context, Result};
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 音乐收藏发布用例
/// * `desc`: `可乐音乐 - 收藏用例`
pub struct MusicCollectAddCase;

impl MusicCollectAddCase {
    //

    ////////

    /// # 1. [CASE] - 添加收藏（或恢复收藏）
    /// * `desc`: `用户将音乐收藏到特定专辑，返回 bool 表示操作状态`
    pub async fn case_add_collect(
        uid: i64,              // 操作者 ID
        music_id: i64,         // 音乐 ID
        album_id: Option<i64>, // 专辑 ID
        ctx: AppContext,       // 应用上下文
    ) -> Result<bool, anyhow::Error> {

        // 调用下层服务，假设下层 save_collect_record 返回 bool (表示是否实际发生了变动/插入)
        let is_changed = ctx
            .music
            .collect
            .add
            .save_collect(uid, music_id, album_id)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD CASE]: ❌️ 音乐收藏操作失败: {}", e))?;

        if is_changed {
            info!(
                "[🗣️ ADD CASE] - ✅️ 音乐收藏成功: uid={}, music_id={}, album_id={:?}",
                uid, music_id, album_id
            );
        } else {
            info!(
                "[🗣️ ADD CASE] - ℹ️ 音乐已经收藏过了: uid={}, music_id={}",
                uid, music_id
            );
        }

        Ok(is_changed)
    }

    ////////

    /// # 2. [CASE] - 取消收藏（逻辑删除）
    /// * `desc`: `用户取消音乐收藏，返回 bool 表示是否实际删除了记录`
    pub async fn case_del_collect(
        uid: i64,        // 操作者 ID
        music_id: i64,   // 音乐 ID
        ctx: AppContext, // 应用上下文
    ) -> Result<bool, anyhow::Error> {

        let album_id = 1;

        // 调用下层的逻辑删除方法（对应我们之前拆分的 delete 仓储/服务），返回 bool
        let is_changed = ctx
            .music
            .collect
            .add
            .un_collect(uid, music_id, Some(album_id)) // 假设下层对应的方法名
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD CASE]: ❌️ 音乐取消收藏失败: {}", e))?;

        if is_changed {
            info!(
                "[🗣️ ADD CASE] - ✅️ 音乐取消收藏成功: uid={}, music_id={}",
                uid, music_id
            );
        } else {
            info!(
                "[🗣️ ADD CASE] - ℹ️ 音乐本来就没有收藏记录: uid={}, music_id={}",
                uid, music_id
            );
        }

        Ok(is_changed)
    }
}

//////// END