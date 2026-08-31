// cola_music/src/case/like/add.rs -- MUSIC - 用例层 - 点赞 - 发布用例
// 2026/8/20 23:50 Created.

////////

use anyhow::{Context, Result};
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 音乐 点赞 发布 用例
/// * `desc`: `音乐 - 点赞用例`
pub struct MusicLikeAddCase;

impl MusicLikeAddCase {
    //

    ////////

    /// # 1. [CASE] - 添加点赞
    /// * `desc`: `用户添加音乐点赞`
    pub async fn case_add_like(
        uid: i64,        // 操作者 ID
        music_id: i64,   // 音乐 ID
        ctx: AppContext, // 应用上下文
    ) -> Result<(), anyhow::Error> {
        // 调用下层端口
        ctx.music
            .like
            .add
            .add_like(uid, music_id)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD CASE]: ❌️ 音乐添加点赞失败: {}", e))?;

        info!(
            "[🗣️ ADD CASE] - ✅️ 音乐添加点赞成功: uid={}, music_id={}",
            uid, music_id
        );

        Ok(())
    }

    ////////

    /// # 2. [CASE] - 取消点赞
    /// * `desc`: `用户取消音乐点赞`
    pub async fn case_del_like(
        uid: i64,        // 操作者 ID
        music_id: i64,   // 音乐 ID
        ctx: AppContext, // 应用上下文
    ) -> Result<(), anyhow::Error> {
        // 调用下层端口
        ctx.music
            .like
            .add
            .un_like(uid, music_id)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD CASE]: ❌️ 音乐取消点赞失败: {}", e))?;

        info!(
            "[🗣️ ADD CASE] - ✅️ 音乐取消点赞成功: uid={}, music_id={}",
            uid, music_id
        );

        Ok(())
    }
}

//////// END